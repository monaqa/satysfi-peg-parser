//! pest parser for SATySFi.

mod peg_parser {
    /// pest parser struct for SATySFi.
    #[derive(Parser)]
    #[grammar = "satysfi.pest"]
    pub struct SatysfiParser;
}

use peg_parser::Rule as SatysfiRule;
use pest::iterators::Pair as PestPair;
use pest::iterators::Pairs as PestPairs;

pub use peg_parser::SatysfiParser;
pub type Rule = SatysfiRule;
pub type Pair<'i> = PestPair<'i, Rule>;
pub type Pairs<'i> = PestPairs<'i, Rule>;

mod tests;
