use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SvartError {
    IllegalValueError(String),
    Other,
}

impl Display for SvartError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            SvartError::IllegalValueError(ref cause) => write!(f, "Illegal value error: {}", cause),
            _ => write!(f, "Other error")
        }
    }
}

impl Error for SvartError {

    fn description(&self) -> &str {
        match *self {
            SvartError::IllegalValueError(ref value) => value,
            _ => "Other error"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        // TODO - possibly implement
        None
    }

}

