use std::borrow::Cow;

pub type Text<'a> = Vec<Inline<'a>>;

#[derive(Debug)]
pub enum Inline<'a> {
    LineBreak,
    Chunk(Cow<'a, str>),
}