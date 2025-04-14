mod decode;
mod encode;

pub use decode::{DecodeError, Decoder, deserialize};
pub use encode::Encoder;

pub const LENGTH_TAG_BYTES: usize = 4;

pub use chorba_macro::Decode;
pub use chorba_macro::Encode;

pub fn encode<T>(data: &T) -> Vec<u8>
where
    T: Encoder,
{
    data.encode()
}

pub fn decode<T>(data: &[u8]) -> Result<T, DecodeError>
where
    T: Decoder<T>,
{
    T::decode(data)
}
