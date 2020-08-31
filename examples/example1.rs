use pest::Parser;
use pest::iterators::{Pair, Pairs};
use satysfi_peg_parser::{SatysfiParser, Rule};

use std::fs;

fn main() {

    let unparsed_file = fs::read_to_string("examples/example1.saty").expect("cannot read file");
    let mut pairs = SatysfiParser::parse(Rule::program, &unparsed_file).unwrap();

    // let header = pairs.next().unwrap();
    // dbg!(&header.into_inner().next().unwrap().into_inner().next());

    // expand_pairs(&mut pairs, 0)
    for p in pairs {
        expand_pairs(p, 0)
    }

}

fn expand_pairs(pair: Pair<Rule>, indent: usize) {
    let space = "  ".repeat(indent);
    let pairs = pair.clone().into_inner();
    if pairs.clone().count() == 0 {
        println!(r#"{}{:?}: "{}""#, space, pair.as_rule(), pair.as_str());
    } else {
        println!("{}[{:?}]", space, pair.as_rule());
    }
    for p in pairs {
        expand_pairs(p, indent + 1);
    }
}
