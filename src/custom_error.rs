use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, Clone)]
pub struct CustomError;

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "an error occurred")
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
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
