use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum SvartError {
    IllegalValueError(&'static str),
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

impl Error for SvartError {}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(SvartError::IllegalValueError("Something went wrong."), "Illegal value error: Something went wrong.")]
    #[case(SvartError::Other, "Other error")]
    fn test_svart_error(#[case] input: SvartError, #[case] expected: &str) {
        assert_eq!(format!("{}", input), expected)
    }
}

