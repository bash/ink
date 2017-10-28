use super::error::ParseError;
use std::error::Error;

pub trait ParserInput<'a> {
    fn into_parser_result(self) -> Result<&'a str, ParseError>;
}

impl<'a, E> ParserInput<'a> for Result<&'a str, E>
where
    E: Error + 'static,
{
    fn into_parser_result(self) -> Result<&'a str, ParseError> {
        self.map_err(ParseError::from_error)
    }
}

impl<'a> ParserInput<'a> for &'a str {
    fn into_parser_result(self) -> Result<&'a str, ParseError> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn from_ok_result() {
        let result: Result<&str, io::Error> = Ok("foo");
        let parser_result = result.into_parser_result();

        assert!(parser_result.is_ok());
        assert_eq!("foo", parser_result.unwrap());
    }

    #[test]
    fn from_err_result() {
        let result: Result<&str, io::Error> = Err(io::ErrorKind::BrokenPipe.into());
        let parser_result = result.into_parser_result();

        assert!(parser_result.is_err());
    }

    #[test]
    fn from_str() {
        let parser_result = "bar".into_parser_result();

        assert!(parser_result.is_ok());
        assert_eq!("bar", parser_result.unwrap());
    }
}