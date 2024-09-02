/// Contains error options that can be encountered while performing the decoding
/// operations.
#[derive(Debug, PartialEq)]
pub enum DecoderError {
    /// Indicates that the decoder received an invalid Huffman code. This should
    /// never happen in the input is encoded according to the HPACK spec.
    InvalidInput,
}
