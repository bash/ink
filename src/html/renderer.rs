use ast::Block;
use error::ParseError;
use super::format::{Format, DefaultFormat};
use super::builders::Builder;
use super::output::Output;
use std::ops;

#[derive(Debug)]
enum OwnedOrBorrowed<'a, T: 'a> {
    Owned(T),
    Borrowed(&'a T),
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
pub struct Renderer<'a, F, I>
where
    F: Format + 'static,
    // TODO: use own error type
    I: Iterator<Item = Result<Block, ParseError>>,
{
    // Not using Cow because Cow would require F to be `Clone`able
    format: OwnedOrBorrowed<'a, F>,
    input: I,
}

impl<'a, T: 'a> ops::Deref for OwnedOrBorrowed<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            &OwnedOrBorrowed::Borrowed(val) => val,
            &OwnedOrBorrowed::Owned(ref val) => &val,
        }
    }
}

impl<'a, I> Renderer<'a, DefaultFormat, I>
where
    I: Iterator<Item = Result<Block, ParseError>>,
{
    ///
    /// Creates a new renderer with the default implementation of `Format`.
    ///
    pub fn new(input: I) -> Self {
        Renderer {
            input,
            format: OwnedOrBorrowed::Owned(DefaultFormat),
        }
    }
}

impl<'a, F, I> Renderer<'a, F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    pub fn with_format(format: &'a F, input: I) -> Self {
        Renderer {
            format: OwnedOrBorrowed::Borrowed(format),
            input,
        }
    }
}

impl<'a, F, I> Iterator for Renderer<'a, F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    type Item = Result<Output, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.input.next()?.and_then(|block| {
            let mut builder = Builder::new();

            match block {
                Block::Heading(level, content) => self.format.heading(&mut builder, level, content),
                _ => unimplemented!(),
            }

            Ok(builder.consume())
        });

        Some(node)
    }
}