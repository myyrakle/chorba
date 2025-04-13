pub mod decode;
pub mod encode;

pub use decode::{Decoder, deserialize};
pub use encode::Encoder;

pub const LENGTH_TAG_BYTES: usize = 4;
