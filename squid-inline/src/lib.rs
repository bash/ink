#![feature(rust_2018_preview, non_exhaustive, decl_macro)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

//! This crate is responsible for parsing Squid's inline syntax (emphasis, links, etc.)

pub mod ast;
pub mod parser;
pub mod replacements;
