#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(clippy::missing_docs_in_private_items)]

//! SATySFi parser.

#[macro_use]
extern crate pest_derive;

mod parser;

pub mod grammar;
