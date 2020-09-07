use pest::Parser;
use pest::Token;
use pest::iterators::Pair;
use pest::Position;
use satysfi_peg_parser::{SatysfiParser, Rule};

use std::fs;

fn main() {

    let unparsed_file = fs::read_to_string("examples/example1.saty").expect("cannot read file");
    let mut pairs = SatysfiParser::parse(Rule::program, &unparsed_file).unwrap();

    // let header = pairs.next().unwrap();
    // dbg!(&header.into_inner().next().unwrap().into_inner().next());

    // for p in pairs {
    //     // expand_pairs(p, 0)
    //     // dbg!(p.tokens());
    //     for t in p.tokens() {
    //         match t {
    //             Token::Start{rule, pos} => {
    //                 dbg!(rule, pos.line_col());
    //             },
    //             Token::End{rule, pos} => {
    //                 dbg!(rule, pos.line_col());
    //             }
    //         }
    //     }
    // }
    for p in pairs {
        expand_pairs(p, 0);
    }

    // let pair: Pair<Rule> = pairs.next().unwrap();
    // for p in pairs.flatten() {
    //     expand_pairs(p, 0)
    // }

    // dbg!(&pairs);
    // let pair: Pair<Rule> = pairs.next().unwrap();
    // dbg!(pair.as_span().start_pos().line_col());

}

fn expand_pairs(pair: Pair<Rule>, indent: usize) {
    let space = "  ".repeat(indent);
    let pairs = pair.clone().into_inner();
    if pairs.clone().count() == 0 {
        println!(
            r#"{}{:?}: "{}" {{start: {:?}, end: {:?}}}"#,
            space, pair.as_rule(), pair.as_str(),
            pair.as_span().start_pos().line_col(),
            pair.as_span().end_pos().line_col()
        );
    } else {
        println!("{}[{:?}]", space, pair.as_rule());
    }
    for p in pairs {
        expand_pairs(p, indent + 1);
    }
}
