use super::error::ParseError;
use std::error::Error;
use std::marker::PhantomData;
use std::borrow::Cow;

pub type ParserInputResult<'a> = Result<Cow<'a, str>, ParseError>;

#[derive(Debug)]
pub struct IntoParserInputIter<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    inner: I,
    _marker: PhantomData<&'a ()>,
}

pub trait IntoParserInput<'a> {
    fn into_parser_input(self) -> ParserInputResult<'a>;
}

impl<'a, E> IntoParserInput<'a> for Result<&'a str, E>
where
    E: Error + 'static,
{
    fn into_parser_input(self) -> ParserInputResult<'a> {
        self.map_err(ParseError::from_error).map(Cow::Borrowed)
    }
}

impl<'a, E> IntoParserInput<'a> for Result<String, E>
where
    E: Error + 'static,
{
    fn into_parser_input(self) -> ParserInputResult<'a> {
        self.map_err(ParseError::from_error).map(Cow::Owned)
    }
}

impl<'a> IntoParserInput<'a> for &'a str {
    fn into_parser_input(self) -> ParserInputResult<'a> {
        Ok(Cow::Borrowed(self))
    }
}

impl<'a> IntoParserInput<'a> for String {
    fn into_parser_input(self) -> ParserInputResult<'a> {
        Ok(Cow::Owned(self))
    }
}

impl<'a, S, I> IntoParserInputIter<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    pub fn new(inner: I) -> Self {
        IntoParserInputIter {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<'a, S, I> Iterator for IntoParserInputIter<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    type Item = ParserInputResult<'a>;

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