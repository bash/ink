#![feature(rust_2018_preview)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

//! This crate includes types that are used by both `squid-inline` and `squid-doc`.

mod span;

pub use self::span::Span;
