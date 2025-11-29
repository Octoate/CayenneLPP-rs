/// Defines all the errors that can occur in this crate.
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// The buffer is too small to add the value
    InsufficientMemory,
    /// The provided value is not representable by CayenneLPP
    OutOfRange,
}