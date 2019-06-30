use std::num::{ParseFloatError, ParseIntError};
use std::{error, fmt};


#[derive(Debug, Clone)]
pub struct CustomError;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occurred")
    }
}

impl error::Error for CustomError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<ParseIntError> for CustomError {
    fn from(_error: ParseIntError) -> Self {
        CustomError {}
    }
}

impl From<ParseFloatError> for CustomError {
    fn from(_error: ParseFloatError) -> Self {
        CustomError {}
    }
}
