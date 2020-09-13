use pest::Parser;
// use pest::Token;
use pest::iterators::Pairs;
// use pest::Position;
use satysfi_peg_parser::{SatysfiParser, Rule};
use satysfi_peg_parser::grammar::{common::Grammar, literal::Literal};

fn main() {

    let mut pairs: Pairs<Rule> = SatysfiParser::parse(Rule::literal, "#``12.5pt``#").unwrap();
    let literal_pair = pairs.next().unwrap();

    dbg!(&literal_pair);

    let hoge = Literal::parse(literal_pair);
    dbg!(hoge);

}
