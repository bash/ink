use super::builder::Builder;
use ast::HeadingLevel;

///
/// HTML generation might need to be customized to invividual use.
/// A `Format` allows to customize output generation (e.g. custom tags, classes, ...)
/// TODO: I need some feedback on the name `Format`
/// TODO: API is only a draft
///
pub trait Format {
    fn heading(&self, builder: &mut Builder, level: HeadingLevel, content: String);
}

pub struct DefaultFormat;

impl Format for DefaultFormat {
    fn heading(&self, builder: &mut Builder, level: HeadingLevel, content: String) {
        let tag = match level {
            HeadingLevel::Level1 => "h1",
            HeadingLevel::Level2 => "h2",
            HeadingLevel::Level3 => "h3",
            HeadingLevel::__NonExhaustive => unreachable!(),
        };

        builder.text_tag(tag, content);
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
            HeadingLevel::Level1,
            "hello world".to_string(),
        );

        assert_eq!("<h1>hello world</h1>", format!("{}", builder.consume()));
    }
}