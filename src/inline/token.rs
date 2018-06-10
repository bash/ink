use crate::span::Span;

#[derive(Debug)]
pub struct TokenTree {
  pub span: Span,
  pub kind: TokenNode,
}

#[derive(Debug)]
pub enum TokenNode {}
