use std::fmt;
use std::error::Error;
use super::format::{Format, DefaultFormat};
use super::builders::Builder;
use super::output::Output;
use super::super::error::ParseError;
use super::super::ast::Block;

#[derive(Debug)]
pub enum RenderError {
    ParseError(ParseError),
}

///
/// # Example
///
/// ```
/// use squid::html::Renderer;
/// use squid::ast::{Block, HeadingLevel};
///
/// let blocks = vec![
///     Ok(Block::Heading(HeadingLevel::Level1, "Hello World".into())),
/// ];
///
/// let mut renderer = Renderer::new(blocks.into_iter());
///
/// for node in renderer {
///     println!("{}", node.unwrap());
/// }
/// ```
///
/// ## Output
/// ```text
/// <h1>hello world</h1>
/// ```
///
#[derive(Debug)]
pub struct Renderer<F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    // Not using Cow because Cow would require F to be `Clone`able
    format: F,
    input: I,
}

impl<I> Renderer<DefaultFormat, I>
where
    I: Iterator<Item = Result<Block, ParseError>>,
{
    ///
    /// Creates a new renderer with the default implementation of `Format`.
    ///
    pub fn new(input: I) -> Self {
        Renderer {
            input,
            format: DefaultFormat,
        }
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for RenderError {
    fn description(&self) -> &str {
        match *self {
            RenderError::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RenderError::ParseError(ref err) => Some(err),
        }
    }
}

impl From<ParseError> for RenderError {
    fn from(err: ParseError) -> Self {
        RenderError::ParseError(err)
    }
}

impl<F, I> Renderer<F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    pub fn with_format(format: F, input: I) -> Self {
        Renderer { format, input }
    }
}

impl<F, I> Iterator for Renderer<F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    type Item = Result<Output, RenderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.input.next()?.and_then(|block| {
            let mut builder = Builder::new();

            match block {
                Block::Heading(level, content) => self.format.heading(&mut builder, level, content),
                Block::Paragraph(text) => self.format.paragraph(&mut builder, text),
                Block::Quote(text) => self.format.quote(&mut builder, text),
                Block::List(list_type, items) => self.format.list(&mut builder, list_type, items),
                _ => unimplemented!(),
            }

            Ok(builder.consume())
        });

        Some(node.map_err(Into::into))
    }
}
