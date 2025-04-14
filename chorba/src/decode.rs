use crate::LENGTH_TAG_BYTES;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Invalid data length")]
    InvalidLength,
    #[error("Decoding error: {0}")]
    Other(String),
}

pub trait Decoder<T: Sized> {
    fn decode(data: &[u8]) -> Result<T, DecodeError>;
}

impl Decoder<String> for String {
    fn decode(data: &[u8]) -> Result<String, DecodeError> {
        Ok(String::from_utf8_lossy(data).to_string())
    }
}

impl Decoder<Vec<u8>> for Vec<u8> {
    fn decode(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
        Ok(data.to_vec())
    }
}

pub fn deserialize(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    if bytes.len() < LENGTH_TAG_BYTES {
        return None;
    }

    let length = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

    // cut length tag
    let bytes = &bytes[LENGTH_TAG_BYTES..];

    if bytes.len() < length {
        return None;
    }

    let current_bytes = &bytes[..length];

    let rest_bytes = &bytes[length..];

    Some((current_bytes, rest_bytes))
}
