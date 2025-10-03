extern crate alloc;

pub mod error;
pub(crate) mod proto;

pub mod common {
    pub use crate::proto::execute::{ExecuteData, ExecuteRequest, ExecuteResponse};
}

#[cfg(feature = "client")]
pub mod client {
    pub use crate::proto::execute::execute_client::ExecuteClient;
}

#[cfg(feature = "server")]
pub mod server {
    pub use crate::proto::execute::execute_server::{Execute, ExecuteServer};
}
