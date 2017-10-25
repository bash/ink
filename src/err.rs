use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InputError(Box<Error>),
    #[doc(hidden)]
    __NonExhaustive,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self {
            &ParseError::InputError(ref err) => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseError::InputError(ref err) => write!(f, "{}", err),
            _ => unreachable!(),
        }
    }
}