extern crate pest;
#[macro_use]
extern crate pest_derive;

// use pest::iterators::Pair;
// use pest::error::Error;

#[derive(Parser)]
#[grammar = "satysfi.pest"]
pub struct SatysfiParser;

pub mod tests;
