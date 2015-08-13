use std::fmt::{Debug, Formatter};
use std::fmt::Error as fmt_Error;

/// Enum type to specify *what* error happened, is used as a
/// member type in `Error`
#[derive(Debug)]
pub enum ErrorType {
    Empty,
    NonNumeric,
    UnsignedOverflow
}

/// The Struct returned in `Result` when an Error is encountered, 
/// likely in from_str when a non-numeric is found or if an empty
/// string is passed. Contains the *type* of error, and the *cause*
pub struct Error {
    error: ErrorType,
    cause: String
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt_Error> {
        write!(f, "type: {:?}, cause: {}", self.error, self.cause)
    }
}

impl Error {
    pub fn new<S>(e: ErrorType, s: S) -> Error where S: Into<String> {
        Error { error: e, cause: s.into() }
    }
}
