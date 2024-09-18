/// Provides encoder input format options.
use alloc::vec::Vec;

use crate::Encoder;

#[derive(Debug)]
pub enum EncoderInput<'a> {
    /// Represents a fully indexed header field.
    Indexed(u32),

    /// Represents a header field where name is represented by an index and the
    /// value is provided in bytes. This format can hold configuration flags.
    IndexedName(u32, Vec<u8>, u8),

    /// Represents a header field where name and value are provided in bytes.
    /// This format can hold configuration flags.
    Literal(&'a [u8], &'a [u8], u8),
}

// impl From<u32> for EncoderInput {
//     fn from(field: u32) -> Self {
//         EncoderInput::Indexed(field)
//     }
// }
// 
// impl From<(u32, Vec<u8>, u8)> for EncoderInput {
//     fn from(field: (u32, Vec<u8>, u8)) -> Self {
//         EncoderInput::IndexedName(field.0, field.1, field.2)
//     }
// }
// 
// impl From<(Vec<u8>, Vec<u8>, u8)> for EncoderInput<'_> {
//     fn from(field: (Vec<u8>, Vec<u8>, u8)) -> Self {
//         EncoderInput::Literal(field.0.as_slice(), field.1.as_slice(), field.2)
//     }
// }

impl<'a> From<(&'a str, &'a str)> for EncoderInput<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        return EncoderInput::Literal(value.0.as_bytes(), value.1.as_bytes(), Encoder::BEST_FORMAT)

    }
}
