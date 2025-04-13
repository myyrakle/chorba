use crate::LENGTH_TAG_BYTES;

pub trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

impl Encoder for String {
    fn encode(&self) -> Vec<u8> {
        serialize_bytes_with_length(self.as_bytes())
    }
}

impl Encoder for Vec<u8> {
    fn encode(&self) -> Vec<u8> {
        serialize_bytes_with_length(self)
    }
}

impl Encoder for &[u8] {
    fn encode(&self) -> Vec<u8> {
        serialize_bytes_with_length(self)
    }
}

pub fn serialize_bytes_with_length(bytes: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len() + LENGTH_TAG_BYTES);

    let len = bytes.len() as u32;

    result.extend_from_slice(&len.to_be_bytes());
    result.extend_from_slice(bytes);

    result
}
