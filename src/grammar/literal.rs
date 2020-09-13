use super::common::{Grammar, Ranged, Rule};
use float_cmp::approx_eq;
use pest::iterators::Pair;
use std::i32;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Unit(Ranged<()>),
    Bool(Ranged<bool>),
    String(Ranged<String>),
    Length(Ranged<Length>),
    Float(Ranged<f64>),
    Int(Ranged<i32>),
}

impl Grammar for Literal {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::unit_const => Literal::Unit(Ranged::wrap((), &inner.as_span())),
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
                let body = Length {
                    value,
                    unit: unit.as_str().to_owned(),
                };
                Ranged::wrap(body, &span)
            }),
            Rule::string_const => Literal::String({
                let span = inner.as_span();
                let mut pairs_inner = inner.into_inner();
                let mut trim_start = true;
                let mut trim_end = true;
                let mut term = pairs_inner.next().unwrap();
                if let Rule::string_omit_space_identifier = term.as_rule() {
                    trim_start = false;
                    term = pairs_inner.next().unwrap();
                }
                if let Some(t) = pairs_inner.next() {
                    match t.as_rule() {
                        Rule::string_omit_space_identifier => {
                            trim_end = false;
                        }
                        _ => unreachable!(),
                    }
                }

                let mut body = term.as_str();
                if trim_start {
                    body = body.trim_start()
                }
                if trim_end {
                    body = body.trim_end()
                }

                Ranged::wrap(body.to_owned(), &span)
            }),
            rule => unreachable!(format!("invalid rule: '{:?}' in rule 'literal'", rule)),
        }
    }
}

impl Literal {
    fn parse_int(pair: Pair<Rule>) -> Ranged<i32> {
        match pair.as_rule() {
            Rule::int_hex_const => {
                let body = pair.as_str();
                let without_prefix = body.trim_start_matches("0x").trim_start_matches("0X");
                let z = i32::from_str_radix(without_prefix, 16).unwrap();
                Ranged::wrap(z, &pair.as_span())
            }
            Rule::int_decimal_const => {
                let body: i32 = pair.as_str().parse().unwrap();
                Ranged::wrap(body, &pair.as_span())
            }
            _ => unreachable!(),
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
    use crate::{Rule, SatysfiParser};
    use pest::Parser;
    use crate::ranged;
    use super::*;
    use crate::grammar::common::Location;
    use pest::iterators::Pairs;

    fn assert_parsed(text: &str, expect: Literal) {
        let mut pairs: Pairs<Rule> = SatysfiParser::parse(Rule::literal, text).unwrap();
        let literal_pair = pairs.next().unwrap();
        let actual = Literal::parse(literal_pair);

        assert_eq!(actual, expect);
    }

    #[test]
    fn parse_literal() {
        assert_parsed("()", Literal::Unit(ranged![(), (1, 3)]));
        assert_parsed("(  )", Literal::Unit(ranged![(), (1, 5)]));

        assert_parsed("true", Literal::Bool(ranged![true, (1, 5)]));
        assert_parsed("false", Literal::Bool(ranged![false, (1, 6)]));

        assert_parsed("123", Literal::Int(ranged![123, (1, 4)]));
        assert_parsed("0x2f1f", Literal::Int(ranged![0x2f1f, (1, 7)]));
        assert_parsed("0x2F1F", Literal::Int(ranged![0x2f1f, (1, 7)]));

        assert_parsed("123.56", Literal::Float(ranged![123.56, (1, 7)]));
        assert_parsed(".56", Literal::Float(ranged![0.56, (1, 4)]));
        assert_parsed("123.", Literal::Float(ranged![123.0, (1, 5)]));

        assert_parsed(
            "0pt",
            Literal::Length(ranged![Length{value: 0.0, unit: "pt".to_owned()}, (1, 4)])
        );
        assert_parsed(
            "0cm",
            Literal::Length(ranged![Length{value: 0.0, unit: "cm".to_owned()}, (1, 4)])
        );
        assert_parsed(
            "0aa",
            Literal::Length(ranged![Length{value: 0.0, unit: "aa".to_owned()}, (1, 4)])
        );
        assert_parsed(
            "12pt",
            Literal::Length(ranged![Length{value: 12.0, unit: "pt".to_owned()}, (1, 5)])
        );
        assert_parsed(
            "12.3pt",
            Literal::Length(ranged![Length{value: 12.3, unit: "pt".to_owned()}, (1, 7)])
        );
        assert_parsed(
            "12.pt",
            Literal::Length(ranged![Length{value: 12.0, unit: "pt".to_owned()}, (1, 6)])
        );
        assert_parsed(
            ".3pt",
            Literal::Length(ranged![Length{value: 0.3, unit: "pt".to_owned()}, (1, 5)])
        );
        assert_parsed(
            ".3pt2",
            Literal::Length(ranged![Length{value: 0.3, unit: "pt2".to_owned()}, (1, 6)])
        );


        assert_parsed("`a`", Literal::String(ranged!["a".to_owned(), (1, 4)]));
        assert_parsed(
            "`` a` ``",
            Literal::String(ranged!["a`".to_owned(), (1, 9)]),
        );
        assert_parsed(
            "`` hoge`fuga ``",
            Literal::String(ranged!["hoge`fuga".to_owned(), (1, 16)]),
        );
        assert_parsed(
            "`` hogefuga ``",
            Literal::String(ranged!["hogefuga".to_owned(), (1, 15)]),
        );
        assert_parsed(
            "#`` hoge`fuga ``",
            Literal::String(ranged![" hoge`fuga".to_owned(), (1, 17)]),
        );
        assert_parsed(
            "#`` hoge`fuga ``#",
            Literal::String(ranged![" hoge`fuga ".to_owned(), (1, 18)]),
        );
        assert_parsed(
            "#```` hoge```fuga ````#",
            Literal::String(ranged![" hoge```fuga ".to_owned(), (1, 24)]),
        );
        assert_parsed(
            "` hoge % fuga `",
            Literal::String(ranged!["hoge % fuga".to_owned(), (1, 16)]),
        );
        assert_parsed(
            "` hoge\nfuga `",
            Literal::String(ranged!["hoge\nfuga".to_owned(), (1, 2), (1, 7)]),
        );
    }
}
