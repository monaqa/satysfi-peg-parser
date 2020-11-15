use super::*;

use crate::parser::{Pairs, SatysfiParser};
use crate::ranged;
use pest::Parser;

/// テスト用の関数。正しくパースされるかどうか検証する。
/// TODO: not fully parsed なときに fail する
fn assert_parsed<'i, T: std::fmt::Debug + Grammar + PartialEq>(text: &'i str, expect: T) {
    let mut pairs: Pairs<'i> = SatysfiParser::parse(T::rule(), text).unwrap();
    let vertical_pair = pairs.next().unwrap();
    let actual = T::parse_pair(vertical_pair);

    assert_eq!(actual, expect);
}

/// テスト用の関数。正しくパースを拒否できるかどうか検証する。
/// TODO: not fully parsed なときに OK とする
fn assert_not_parsed<T: std::fmt::Debug + Grammar + PartialEq>(text: &str) {
    if SatysfiParser::parse(T::rule(), text).is_ok() {
        panic!(format!(
            r#"Text "{}" is successfully parsed by "{:?}" rule!"#,
            text,
            T::rule()
        ));
    }
}

#[test]
fn parse_literal() {
    assert_parsed("()", Literal::Unit(ranged![(), (1, 3)]));
    assert_parsed("(  )", Literal::Unit(ranged![(), (1, 5)]));

    assert_parsed("true", Literal::Bool(ranged![true, (1, 5)]));
    assert_parsed("false", Literal::Bool(ranged![false, (1, 6)]));
    assert_not_parsed::<Literal>("True");
    assert_not_parsed::<Literal>("TRUE");

    assert_parsed("123", Literal::Int(ranged![123, (1, 4)]));
    assert_parsed("0x123", Literal::Int(ranged![0x123, (1, 6)]));
    assert_parsed("0x2f1f", Literal::Int(ranged![0x2f1f, (1, 7)]));
    assert_parsed("0x2F1F", Literal::Int(ranged![0x2f1f, (1, 7)]));
    // assert_not_parsed::<Literal>("1_000");
    // assert_not_parsed::<Literal>("0X2f1f");
    // assert_not_parsed::<Literal>("0X2F1F");

    assert_parsed("123.56", Literal::Float(ranged![123.56, (1, 7)]));
    assert_parsed(".56", Literal::Float(ranged![0.56, (1, 4)]));
    assert_parsed("123.", Literal::Float(ranged![123.0, (1, 5)]));

    assert_parsed(
        "0pt",
        Literal::Length(ranged![
            Length {
                value: 0.0,
                unit: "pt".to_owned()
            },
            (1, 4)
        ]),
    );
    assert_parsed(
        "0cm",
        Literal::Length(ranged![
            Length {
                value: 0.0,
                unit: "cm".to_owned()
            },
            (1, 4)
        ]),
    );
    assert_parsed(
        "0aa",
        Literal::Length(ranged![
            Length {
                value: 0.0,
                unit: "aa".to_owned()
            },
            (1, 4)
        ]),
    );
    assert_parsed(
        "12pt",
        Literal::Length(ranged![
            Length {
                value: 12.0,
                unit: "pt".to_owned()
            },
            (1, 5)
        ]),
    );
    assert_parsed(
        "12.3pt",
        Literal::Length(ranged![
            Length {
                value: 12.3,
                unit: "pt".to_owned()
            },
            (1, 7)
        ]),
    );
    assert_parsed(
        "12.pt",
        Literal::Length(ranged![
            Length {
                value: 12.0,
                unit: "pt".to_owned()
            },
            (1, 6)
        ]),
    );
    assert_parsed(
        ".3pt",
        Literal::Length(ranged![
            Length {
                value: 0.3,
                unit: "pt".to_owned()
            },
            (1, 5)
        ]),
    );
    assert_parsed(
        ".3pt2",
        Literal::Length(ranged![
            Length {
                value: 0.3,
                unit: "pt2".to_owned()
            },
            (1, 6)
        ]),
    );

    assert_parsed("` `", Literal::String(ranged!["".to_owned(), (1, 4)]));
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
    assert_not_parsed::<Literal>("``");
}
