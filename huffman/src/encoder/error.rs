/// Contains error options that can be encountered while performing the encoding
/// operations.
#[derive(Debug, PartialEq)]
pub enum EncoderError {
    /// Indicates that the encoder received an invalid ASCII character. Note
    /// that only ASCII characters provided in the HPACK spec should be used.
    InvalidInput,
}
