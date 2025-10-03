pub mod execute {
    use crate::error::NihilityRpcError;

    tonic::include_proto!("nihility.execute");

    #[derive(Debug, Clone)]
    pub enum ExecuteData {
        String(String),
    }

    impl TryFrom<ExecuteRequest> for ExecuteData {
        type Error = NihilityRpcError;

        fn try_from(value: ExecuteRequest) -> Result<Self, Self::Error> {
            match value.r#type() {
                ExecuteType::String => Ok(ExecuteData::String(String::from_utf8(value.payload)?)),
            }
        }
    }

    impl From<ExecuteData> for ExecuteRequest {
        fn from(value: ExecuteData) -> Self {
            match value {
                ExecuteData::String(string_value) => ExecuteRequest {
                    r#type: ExecuteType::String.into(),
                    payload: string_value.into_bytes(),
                },
            }
        }
    }

    impl TryFrom<ExecuteResponse> for ExecuteData {
        type Error = NihilityRpcError;

        fn try_from(value: ExecuteResponse) -> Result<Self, Self::Error> {
            match value.r#type() {
                ExecuteType::String => Ok(ExecuteData::String(String::from_utf8(value.payload)?)),
            }
        }
    }

    impl From<ExecuteData> for ExecuteResponse {
        fn from(value: ExecuteData) -> Self {
            match value {
                ExecuteData::String(string_value) => ExecuteResponse {
                    r#type: ExecuteType::String.into(),
                    payload: string_value.into_bytes(),
                },
            }
        }
    }
}
