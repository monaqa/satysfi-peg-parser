use pest::Parser;
// use pest::Token;
use pest::iterators::Pairs;
// use pest::Position;
use satysfi_peg_parser::{SatysfiParser, Rule};
use satysfi_peg_parser::grammar::{common::Grammar, literal::Literal};

fn main() {
    let hoge = Literal::parse("#``12.5pt``#");
    dbg!(hoge);
}
