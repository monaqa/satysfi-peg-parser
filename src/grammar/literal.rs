use pest::iterators::Pair;
use super::common::{Ranged, Grammar, Rule};
use std::i32;
use float_cmp::approx_eq;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Unit(Ranged<()>),
    Bool(Ranged<bool>),
    String(Ranged<String>),
    Length(Ranged<Length>),
    Float(Ranged<f64>),
    Int(Ranged<i32>)
}

impl Grammar for Literal {

    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {

        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::unit_const => Literal::Unit(
                Ranged::wrap((), &inner.as_span())
            ),
            Rule::bool_const => Literal::Bool({
                let body = match inner.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => unreachable!(),
                };
                Ranged::wrap(body, &inner.as_span())
            }),
            Rule::int_const => Literal::Int({
                let inner = inner.into_inner().next().unwrap();
                Literal::parse_int(inner)
            }),
            Rule::float_const => Literal::Float({
                let body = inner.as_str().parse().unwrap();
                Ranged::wrap(body, &inner.as_span())
            }),
            Rule::length_const => Literal::Length({
                let span = inner.as_span();
                let mut pairs_inner = inner.into_inner();
                let digit = pairs_inner.next().unwrap();
                let unit = pairs_inner.next().unwrap();
                let value: f64 = digit.as_str().parse().unwrap();
                let body = Length { value, unit: unit.as_str().to_owned() };
                Ranged::wrap(body, &span)
            }),
            Rule::string_const => Literal::String({
                let span = inner.as_span();
                let mut pairs_inner = inner.into_inner();
                let mut term = pairs_inner.next().unwrap();
                if let Rule::string_omit_space_identifier = term.as_rule() {
                    term = pairs_inner.next().unwrap();
                }
                let body = term.as_str().to_owned();
                Ranged::wrap(body, &span)
            }),
            rule => unreachable!(format!(
                    "invalid rule: '{:?}' in rule 'literal'", rule))
        }
    }

}

impl Literal {

    fn parse_int(pair: Pair<Rule>) -> Ranged<i32> {
        match pair.as_rule() {
            Rule::int_hex_const => {
                let body = pair.as_str();
                let without_prefix = body.trim_start_matches("0x")
                    .trim_start_matches("0X");
                let z = i32::from_str_radix(without_prefix, 16).unwrap();
                Ranged::wrap(z, &pair.as_span())
            }
            Rule::int_decimal_const => {
                let body: i32 = pair.as_str().parse().unwrap();
                Ranged::wrap(body, &pair.as_span())
            }
            _ => unreachable!()
        }
    }

}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Length {
    pub value: f64,
    pub unit: String,
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::{SatysfiParser, Rule, grammar::common::Location};
    use super::*;
    use pest::iterators::Pairs;

    fn assert_parsed(text: &str, expect: Literal) {
        let mut pairs: Pairs<Rule> = SatysfiParser::parse(Rule::literal, text).unwrap();
        let literal_pair = pairs.next().unwrap();
        let actual = Literal::parse(literal_pair);

        assert_eq!(actual, expect);
    }

    #[test]
    fn parse_literal() {
        assert_parsed("12", Literal::Int(Ranged{start: Location{row: 1, col: 1}, end: Location{row: 1, col: 3}, body: 12}))
    }
}
