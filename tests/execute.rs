use nihility_rpc::client::ExecuteClient;
use nihility_rpc::common::{ExecuteData, ExecuteRequest, ExecuteResponse};
use nihility_rpc::server::ExecuteServer;
use std::pin::Pin;
use time::format_description::well_known::Iso8601;
use tokio::spawn;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

const TEST_PORT: usize = 8000;

type StreamResp = Pin<Box<dyn Stream<Item = Result<ExecuteResponse, Status>> + Send>>;

#[derive(Default)]
pub struct TestExecuteServer;

#[tonic::async_trait]
impl nihility_rpc::server::Execute for TestExecuteServer {
    async fn execute(
        &self,
        request: Request<ExecuteRequest>,
    ) -> Result<Response<ExecuteResponse>, Status> {
        let req: ExecuteData = request
            .into_inner()
            .try_into()
            .expect("execute request data format error");
        info!("Executing request: {:?}", req);
        Ok(Response::new(req.into()))
    }

    type ExecuteStreamOutStream = StreamResp;

    async fn execute_stream_out(
        &self,
        request: Request<ExecuteRequest>,
    ) -> Result<Response<Self::ExecuteStreamOutStream>, Status> {
        let req: ExecuteData = request
            .into_inner()
            .try_into()
            .expect("execute request data format error");
        info!("Executing request: {:?}", req);
        let (tx, rx) = mpsc::channel(10);
        for i in 0..10 {
            info!("stream out index： {}", i);
            tx.send(Ok(ExecuteData::String(format!("test, index: {}", i)).into()))
                .await
                .expect("stream out failed");
        }
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::ExecuteStreamOutStream
        ))
    }

    type ExecuteStreamStream = StreamResp;

    async fn execute_stream(
        &self,
        request: Request<Streaming<ExecuteRequest>>,
    ) -> Result<Response<Self::ExecuteStreamStream>, Status> {
        let mut req_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(10);
        spawn(async move {
            while let Some(req) = req_stream.next().await {
                match req {
                    Ok(ok_req) => {
                        let req_data: ExecuteData = ok_req
                            .try_into()
                            .expect("execute request data format error");
                        info!("execute_stream Executing request: {:?}", req_data);
                        tx.send(Ok(req_data.into()))
                            .await
                            .expect("stream out failed");
                    }
                    Err(err_req) => {
                        error!("execute_stream Executing request error: {:?}", err_req);
                    }
                }
            }
        });
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::ExecuteStreamOutStream
        ))
    }
}

#[tokio::test]
async fn test_execute() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_thread_ids(true)
                .with_target(true)
                .with_timer(LocalTime::new(Iso8601::DATE_TIME_OFFSET))
                .with_filter(LevelFilter::DEBUG),
        )
        .init();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let server_handle = spawn(async move {
        test_execute_server(shutdown_rx).await;
    });
    test_execute_client(shutdown_tx).await;
    server_handle.await.expect("server failed");
}

async fn test_execute_server(shutdown_rx: oneshot::Receiver<()>) {
    let addr = format!("[::1]:{TEST_PORT}")
        .parse()
        .expect("parse addr error");
    info!("start server");
    Server::builder()
        .add_service(ExecuteServer::new(TestExecuteServer::default()))
        .serve_with_shutdown(addr, async move {
            let _ = shutdown_rx.await;
        })
        .await
        .expect("server error");
}

async fn test_execute_client(shutdown_tx: oneshot::Sender<()>) {
    let addr = format!("http://[::1]:{TEST_PORT}");
    let mut client = ExecuteClient::connect(addr).await.expect("connect error");
    info!("client connected");
    info!("test service execute function");
    let execute_resp: ExecuteData = client
        .execute(Request::new(ExecuteData::String("test".to_string()).into()))
        .await
        .expect("execute response error")
        .into_inner()
        .try_into()
        .expect("execute response format error");
    info!("execute response: {:?}", execute_resp);
    info!("test service execute_stream_out function");
    let mut execute_stream_out_resp = client
        .execute_stream_out(Request::new(ExecuteData::String("test".to_string()).into()))
        .await
        .expect("execute_stream_out response error")
        .into_inner();
    while let Some(Ok(resp_chunk)) = execute_stream_out_resp.next().await {
        info!(
            "execute_stream_out_resp_chunk: {:?}",
            ExecuteData::try_from(resp_chunk).expect("execute_stream_out_resp_chunk format error")
        );
    }
    info!("test service execute_stream function");
    let req_stream = tokio_stream::iter(1..10)
        .map(|i| ExecuteRequest::from(ExecuteData::String(format!("test {}", i))));
    let mut execute_stream_resp = client
        .execute_stream(req_stream)
        .await
        .expect("execute_stream response error")
        .into_inner();
    while let Some(Ok(resp_chunk)) = execute_stream_resp.next().await {
        info!(
            "execute_stream_resp_chunk: {:?}",
            ExecuteData::try_from(resp_chunk).expect("execute_stream_resp_chunk format error")
        );
    }
    info!("test success");
    let _ = shutdown_tx.send(());
}
