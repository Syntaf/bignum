use std::fmt::{Debug, Formatter};
use std::fmt::Error as fmt_Error;

#[derive(Debug)]
pub enum ErrorType {
    Empty,
    NonNumeric
}

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
