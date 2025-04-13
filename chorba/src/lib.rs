pub trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Invalid data length")]
    InvalidLength,
    #[error("Invalid data format")]
    InvalidFormat,
    #[error("Decoding error: {0}")]
    Other(String),
}

pub trait Decoder<T: Sized> {
    fn decode(data: &[u8]) -> Result<T, DecodeError>;
}
