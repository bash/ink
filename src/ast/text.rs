pub type Text = Vec<Inline>;

#[derive(Debug, PartialEq, Eq)]
pub enum Inline {
    LineBreak,
    Chunk(String),
}
