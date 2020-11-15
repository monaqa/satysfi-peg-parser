// use pest::Parser;
// use pest::iterators::Pair;
// use satysfi_peg_parser::satysfi::{SatysfiParser, Rule};
// 
// use std::fs;

fn main() {

    // let unparsed_file = fs::read_to_string("examples/example1.saty").expect("cannot read file");
    // let pairs = SatysfiParser::parse(Rule::program, &unparsed_file).unwrap();
    // 
    // 
    // for p in pairs {
    //     expand_pairs(p, 0);
    // }

}

// fn expand_pairs(pair: Pair<Rule>, indent: usize) {
//     let space = "  ".repeat(indent);
//     let pairs = pair.clone().into_inner();
//     if pairs.clone().count() == 0 {
//         println!(
//             r#"{}{:?}: "{}" {{start: {:?}, end: {:?}}}"#,
//             space, pair.as_rule(), pair.as_str(),
//             pair.as_span().start_pos().line_col(),
//             pair.as_span().end_pos().line_col()
//         );
//     } else {
//         println!("{}[{:?}]", space, pair.as_rule());
//     }
//     for p in pairs {
//         expand_pairs(p, indent + 1);
//     }
// }
