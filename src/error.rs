use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    /// Wraps any errors that might arise from the input
    InputError(Box<Error>),
}

impl ParseError {
    pub(crate) fn from_error<E>(err: E) -> Self
    where
        E: Error + 'static,
    {
        ParseError::InputError(Box::new(err))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseError::InputError(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self {
            &ParseError::InputError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ParseError::InputError(ref err) => Some(err.as_ref()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn description_works() {
        let err = ParseError::from_error(io::Error::from(io::ErrorKind::BrokenPipe));
        let io_err = io::Error::from(io::ErrorKind::BrokenPipe);

        assert_eq!(err.description(), io_err.description());
    }

    #[test]
    fn display_works() {
        let err = ParseError::from_error(io::Error::from(io::ErrorKind::BrokenPipe));
        let io_err = io::Error::from(io::ErrorKind::BrokenPipe);

        assert_eq!(format!("{}", io_err), format!("{}", err));
    }
}