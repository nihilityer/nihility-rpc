extern crate alloc;

pub mod error;
pub(crate) mod proto;

pub mod common {
    pub use crate::proto::execute::{ExecuteData, ExecuteRequest, ExecuteResponse};
}

pub mod client {
    pub use crate::proto::execute::execute_client::ExecuteClient;
}

pub mod server {
    pub use crate::proto::execute::execute_server::{Execute, ExecuteServer};
}
