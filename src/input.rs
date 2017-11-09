use super::error::ParseError;
use std::error::Error;

pub type ParserInputResult = Result<String, ParseError>;

#[derive(Debug)]
pub struct IntoParserInputIter<S, I>
where
    S: IntoParserInput,
    I: Iterator<Item = S>,
{
    inner: I,
}

pub trait IntoParserInput {
    fn into_parser_input(self) -> ParserInputResult;
}

impl<'a, E> IntoParserInput for Result<&'a str, E>
where
    E: Error + 'static,
{
    fn into_parser_input(self) -> ParserInputResult {
        self.map_err(ParseError::from_error).map(|val| val.into())
    }
}

impl<E> IntoParserInput for Result<String, E>
where
    E: Error + 'static,
{
    fn into_parser_input(self) -> ParserInputResult {
        self.map_err(ParseError::from_error)
    }
}

impl<'a> IntoParserInput for &'a str {
    fn into_parser_input(self) -> ParserInputResult {
        Ok(self.into())
    }
}

impl IntoParserInput for String {
    fn into_parser_input(self) -> ParserInputResult {
        Ok(self)
    }
}

impl<S, I> IntoParserInputIter<S, I>
where
    S: IntoParserInput,
    I: Iterator<Item = S>,
{
    pub fn new(inner: I) -> Self {
        IntoParserInputIter { inner }
    }
}

impl<S, I> Iterator for IntoParserInputIter<S, I>
where
    S: IntoParserInput,
    I: Iterator<Item = S>,
{
    type Item = ParserInputResult;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_parser_input())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn from_ok_result() {
        let result: Result<&str, io::Error> = Ok("foo");
        let parser_result = result.into_parser_input();

        assert!(parser_result.is_ok());
        assert_eq!("foo", parser_result.unwrap());
    }

    #[test]
    fn from_err_result() {
        let result: Result<&str, io::Error> = Err(io::ErrorKind::BrokenPipe.into());
        let parser_result = result.into_parser_input();

        assert!(parser_result.is_err());
    }

    #[test]
    fn from_str() {
        let parser_result = "bar".into_parser_input();

        assert!(parser_result.is_ok());
        assert_eq!("bar", parser_result.unwrap());
    }
}
