use pest::Parser;
use satysfi_peg_parser::{SatysfiParser, Rule};

use std::fs;

fn main() {

    let unparsed_file = fs::read_to_string("examples/example1.saty").expect("cannot read file");
    let pairs = SatysfiParser::parse(Rule::program, &unparsed_file);

    dbg!(&pairs);

}
