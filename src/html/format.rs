use super::builders::Builder;
use ast::{Heading, HeadingLevel};
use std::fmt::Debug;

///
/// HTML generation might need to be customized to invividual use.
/// A `Format` allows to customize output generation (e.g. custom tags, classes, ...)
/// TODO: I need some feedback on the name `Format`
/// TODO: API is only a draft
///
pub trait Format<'a>: Debug {
    fn heading(&self, builder: &mut Builder<'a>, heading: Heading<'a>);
}

#[derive(Debug)]
pub struct DefaultFormat;

impl<'a> Format<'a> for DefaultFormat {
    fn heading(&self, builder: &mut Builder<'a>, heading: Heading<'a>) {
        let (level, content) = heading.consume();

        let tag = match level {
            HeadingLevel::Level1 => "h1",
            HeadingLevel::Level2 => "h2",
            HeadingLevel::Level3 => "h3",
            HeadingLevel::__NonExhaustive => unreachable!(),
        };

        builder.tag_start(tag).finish().text(content).tag_end(tag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::HeadingLevel;

    #[test]
    fn default_heading() {
        let format = DefaultFormat;
        let mut builder = Builder::new();

        format.heading(
            &mut builder,
            Heading::new(HeadingLevel::Level1, "hello world"),
        );

        format.heading(&mut builder, Heading::new(HeadingLevel::Level2, "level 2"));

        format.heading(&mut builder, Heading::new(HeadingLevel::Level3, "level 3"));

        assert_eq!(
            "<h1>hello world</h1><h2>level 2</h2><h3>level 3</h3>",
            format!("{}", builder.consume())
        );
    }
}