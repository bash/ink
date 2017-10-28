use ast::Block;
use error::ParseError;
use super::format::Format;
use super::builder::{Builder, Output};

///
/// # Example
///
/// ```
/// use squid::html::{Generator, DefaultFormat};
/// use squid::ast::{Block, HeadingLevel};
///
/// let blocks = vec![
///     Ok(Block::Heading {
///         level: HeadingLevel::Level1,
///         content: "Hello World".to_string(),
///     }),
/// ];
///
/// let mut generator = Generator::new(&DefaultFormat, blocks.into_iter());
///
/// for node in generator {
///     println!("{}", node.unwrap());
/// }
/// ```
///
/// ## Output
/// ```text
/// <h1>hello world</h1>
/// ```
///
pub struct Generator<'a, 'b, F, I>
where
    F: Format + 'static,
    // TODO: use own error type
    I: Iterator<Item = Result<Block, ParseError>>,
{
    format: &'a F,
    input: I,
    _marker: ::std::marker::PhantomData<&'b ()>,
}

impl<'a, 'b, F, I> Generator<'a, 'b, F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    pub fn new(format: &'a F, input: I) -> Self {
        Generator {
            format,
            input,
            _marker: ::std::marker::PhantomData,
        }
    }
}

impl<'a, 'b, F, I> Iterator for Generator<'a, 'b, F, I>
where
    F: Format + 'static,
    I: Iterator<Item = Result<Block, ParseError>>,
{
    type Item = Result<Output<'b>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.input.next()?.and_then(|block| {
            let mut builder = Builder::new();

            match block {
                Block::Heading { level, content } => {
                    self.format.heading(&mut builder, level, content)
                }
                _ => unimplemented!(),
            }

            Ok(builder.consume())
        });

        Some(node)
    }
}