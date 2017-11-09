#![feature(non_exhaustive)]

#[macro_use]
extern crate matches;

mod block_tokenizer;
mod block_parser;
mod parser;
mod constants;
mod tokens;
mod input;
pub mod ast;
pub mod error;
pub mod html;

pub use parser::Parser;
