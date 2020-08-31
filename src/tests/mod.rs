mod expr;
mod header;
mod horizontal;
mod literal;
mod program;
mod statement;
mod vertical;

#[cfg(test)]
pub mod common{

    use pest::Parser;
    use pest::iterators::Pairs;
    use crate::{SatysfiParser, Rule};

    pub fn assert_success(rule: Rule, text: &str) {
        let pairs: Pairs<Rule> = SatysfiParser::parse(rule, text).unwrap();
        if pairs.as_str() != text {
            panic!(format!(
                    "Not fully consumed. original: {}, consumed: {}",
                    text, pairs.as_str()))
        }
    }

    pub fn assert_fail(rule: Rule, text: &str) {
        if let Ok(pairs) = SatysfiParser::parse(rule, text) {
            if pairs.as_str() == text {
                panic!(format!(
                        "Successfully parsed and fully consumed!: {}", text))
            }
        }
    }

}
