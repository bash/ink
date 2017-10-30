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
/// use squid::ast::{Heading, HeadingLevel, BlockInner};
///
/// let blocks = vec![
///     Ok(Heading::new(HeadingLevel::Level1, "Hello World").wrap()),
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
pub struct Renderer<'a, 'b: 'a, F, I>
where
    F: Format<'a> + 'static,
    // TODO: use own error type
    I: Iterator<Item = Result<Block<'a>, ParseError>>,
{
    // Not using Cow because Cow would require F to be `Clone`able
    format: OwnedOrBorrowed<'a, F>,
    input: I,
    _marker: ::std::marker::PhantomData<&'b ()>,
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

impl<'a, 'b: 'a, I> Renderer<'a, 'b, DefaultFormat, I>
where
    I: Iterator<Item = Result<Block<'a>, ParseError>>,
{
///
/// Creates a new renderer with the default implementation of `Format`.
///
    pub fn new(input: I) -> Self {
        Renderer {
            input,
            format: OwnedOrBorrowed::Owned(DefaultFormat),
            _marker: ::std::marker::PhantomData,
        }
    }
}

impl<'a, 'b: 'a, F, I> Renderer<'a, 'b, F, I>
where
    F: Format<'a> + 'static,
    I: Iterator<Item = Result<Block<'a>, ParseError>>,
{
    pub fn with_format(format: &'a F, input: I) -> Self {
        Renderer {
            format: OwnedOrBorrowed::Borrowed(format),
            input,
            _marker: ::std::marker::PhantomData,
        }
    }
}

impl<'a, 'b: 'a, F, I> Iterator for Renderer<'a, 'b, F, I>
where
    F: Format<'a> + 'static,
    I: Iterator<
        Item = Result<
            Block<'a>,
            ParseError,
        >,
    >,
{
    type Item = Result<Output<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.input.next()?.and_then(|block| {
            let mut builder = Builder::new();

            match block {
                Block::Heading(inner) => self.format.heading(&mut builder, inner),
                _ => unimplemented!(),
            }

            Ok(builder.consume())
        });

        Some(node)
    }
}