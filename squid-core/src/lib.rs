#![feature(rust_2018_preview)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

//! This crate includes types that are used by both `squid-inline` and `squid-doc`.

#[cfg(feature = "use_serde")]
#[macro_use]
extern crate serde_derive;

pub mod span;
