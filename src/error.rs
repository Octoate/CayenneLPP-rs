/// Defines all the errors that can occur in this crate.
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// The buffer is too small to add the value
    InsufficientMemory,
    /// The provided value is not representable by CayenneLPP
    OutOfRange,
    /// The provided byte buffer didn't contain enough bytes
    /// to unpack the next scalar
    BufferUnderrun,
    /// The provided type code is either invalid or
    /// not handled by this library
    UnhandledType(u8),
    
}