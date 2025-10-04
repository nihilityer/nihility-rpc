use crate::error::NihilityRpcError;
use prost::Message;
use std::ops::Deref;

tonic::include_proto!("nihility.execute");

#[derive(Debug, Clone)]
pub enum AudioEndiannessType {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone)]
pub struct AudioData {
    pub sample_rate: u32,
    pub sample_size: u32,
    pub channels: u32,
    pub sign: bool,
    pub byte_order: AudioEndiannessType,
    pub data: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum ExecuteData {
    String(String),
    Audio(AudioData),
}

impl TryFrom<ExecuteRequest> for ExecuteData {
    type Error = NihilityRpcError;

    fn try_from(value: ExecuteRequest) -> Result<Self, Self::Error> {
        match value.r#type() {
            ExecuteType::String => Ok(ExecuteData::String(String::from_utf8(value.payload)?)),
            ExecuteType::Audio => Ok(ExecuteData::Audio(
                AudioChunk::decode(value.payload.deref())?.into(),
            )),
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
            ExecuteData::Audio(audio_data) => ExecuteRequest {
                r#type: ExecuteType::Audio.into(),
                payload: AudioChunk::from(audio_data).encode_to_vec(),
            },
        }
    }
}

impl TryFrom<ExecuteResponse> for ExecuteData {
    type Error = NihilityRpcError;

    fn try_from(value: ExecuteResponse) -> Result<Self, Self::Error> {
        match value.r#type() {
            ExecuteType::String => Ok(ExecuteData::String(String::from_utf8(value.payload)?)),
            ExecuteType::Audio => Ok(ExecuteData::Audio(
                AudioChunk::decode(value.payload.deref())?.into(),
            )),
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
            ExecuteData::Audio(audio_data) => ExecuteResponse {
                r#type: ExecuteType::Audio.into(),
                payload: AudioChunk::from(audio_data).encode_to_vec(),
            },
        }
    }
}

impl From<AudioData> for AudioChunk {
    fn from(value: AudioData) -> Self {
        AudioChunk {
            sample_rate: value.sample_rate,
            sample_size: value.sample_size,
            channels: value.channels,
            sign: value.sign,
            byte_order: match value.byte_order {
                AudioEndiannessType::LittleEndian => AudioEndianness::Little.into(),
                AudioEndiannessType::BigEndian => AudioEndianness::Big.into(),
            },
            data: value.data,
        }
    }
}

impl From<AudioChunk> for AudioData {
    fn from(value: AudioChunk) -> Self {
        AudioData {
            sample_rate: value.sample_rate,
            sample_size: value.sample_size,
            channels: value.channels,
            sign: value.sign,
            byte_order: match value.byte_order() {
                AudioEndianness::Big => AudioEndiannessType::BigEndian,
                AudioEndianness::Little => AudioEndiannessType::LittleEndian,
            },
            data: value.data,
        }
    }
}

impl Default for AudioData {
    fn default() -> Self {
        AudioData {
            sample_rate: 32000,
            sample_size: 16,
            channels: 1,
            sign: true,
            byte_order: AudioEndiannessType::LittleEndian,
            data: vec![],
        }
    }
}