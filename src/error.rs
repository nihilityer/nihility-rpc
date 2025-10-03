pub type Result<T> = core::result::Result<T, NihilityRpcError>;

#[derive(thiserror::Error, Debug)]
pub enum NihilityRpcError {
    #[error(transparent)]
    ParseString(#[from] alloc::string::FromUtf8Error),
}
