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
fn parse_stage() {
    assert_parsed("@stage: 0\n", Stage::Stage0);
    assert_parsed("@stage:0\n", Stage::Stage0);
    assert_parsed("@stage: 1  \n", Stage::Stage1);
    assert_parsed("@stage: persistent\n", Stage::Persistent);
    assert_not_parsed::<Stage>("@stage: 2\n");
}

#[test]
fn parse_header() {
    assert_parsed(
        "@require: code\n",
        Header::Require(ranged!["code".to_string(), (11, 15)]),
    );
    assert_parsed(
        "@require:code\n",
        Header::Require(ranged!["code".to_string(), (10, 14)]),
    );
    assert_parsed(
        "@require: base/ref\n",
        Header::Require(ranged!["base/ref".to_string(), (11, 19)]),
    );
    assert_parsed(
        "@require: $today\n",
        Header::Require(ranged!["$today".to_string(), (11, 17)]),
    );
    assert_parsed(
        "@import: hoge\n",
        Header::Import(ranged!["hoge".to_string(), (10, 14)]),
    );
    assert_parsed(
        "@import: ../../fuga\n",
        Header::Import(ranged!["../../fuga".to_string(), (10, 20)]),
    );
    assert_parsed(
        "@import: ../../fuga base\n",
        Header::Import(ranged!["../../fuga base".to_string(), (10, 25)]),
    );
    assert_not_parsed::<Header>("@require : base\n");
}

#[test]
fn parse_literal() {
    assert_parsed("()", Literal::Unit);
    assert_parsed("(  )", Literal::Unit);

    assert_parsed("true", Literal::Bool(true));
    assert_parsed("false", Literal::Bool(false));
    assert_not_parsed::<Literal>("True");
    assert_not_parsed::<Literal>("TRUE");

    assert_parsed("123", Literal::Int(123));
    assert_parsed("0x123", Literal::Int(0x123));
    assert_parsed("0x2f1f", Literal::Int(0x2f1f));
    assert_parsed("0x2F1F", Literal::Int(0x2f1f));
    // assert_not_parsed::<Literal>("1_000");
    // assert_not_parsed::<Literal>("0X2f1f");
    // assert_not_parsed::<Literal>("0X2F1F");

    assert_parsed("123.56", Literal::Float(123.56));
    assert_parsed(".56", Literal::Float(0.56));
    assert_parsed("123.", Literal::Float(123.0));

    assert_parsed(
        "0pt",
        Literal::Length(Length {
            value: ranged![0.0, (1, 2)],
            unit: ranged!["pt".to_owned(), (2, 4)],
        }),
    );
    assert_parsed(
        "0cm",
        Literal::Length(Length {
            value: ranged![0.0, (1, 2)],
            unit: ranged!["cm".to_owned(), (2, 4)],
        }),
    );
    assert_parsed(
        "0aa",
        Literal::Length(Length {
            value: ranged![0.0, (1, 2)],
            unit: ranged!["aa".to_owned(), (2, 4)],
        }),
    );
    assert_parsed(
        "12pt",
        Literal::Length(Length {
            value: ranged![12.0, (1, 3)],
            unit: ranged!["pt".to_owned(), (3, 5)],
        }),
    );
    assert_parsed(
        "12.3pt",
        Literal::Length(Length {
            value: ranged![12.3, (1, 5)],
            unit: ranged!["pt".to_owned(), (5, 7)],
        }),
    );
    assert_parsed(
        "12.pt",
        Literal::Length(Length {
            value: ranged![12.0, (1, 4)],
            unit: ranged!["pt".to_owned(), (4, 6)],
        }),
    );
    assert_parsed(
        ".3pt",
        Literal::Length(Length {
            value: ranged![0.3, (1, 3)],
            unit: ranged!["pt".to_owned(), (3, 5)],
        }),
    );
    assert_parsed(
        ".3pt2",
        Literal::Length(Length {
            value: ranged![0.3, (1, 3)],
            unit: ranged!["pt2".to_owned(), (3, 6)],
        }),
    );

    assert_parsed("` `", Literal::String("".to_owned()));
    assert_parsed("`a`", Literal::String("a".to_owned()));
    assert_parsed("`` a` ``", Literal::String("a`".to_owned()));
    assert_parsed("`` hoge`fuga ``", Literal::String("hoge`fuga".to_owned()));
    assert_parsed("`` hogefuga ``", Literal::String("hogefuga".to_owned()));
    assert_parsed("#`` hoge`fuga ``", Literal::String(" hoge`fuga".to_owned()));
    assert_parsed(
        "#`` hoge`fuga ``#",
        Literal::String(" hoge`fuga ".to_owned()),
    );
    assert_parsed(
        "#```` hoge```fuga ````#",
        Literal::String(" hoge```fuga ".to_owned()),
    );
    assert_parsed("` hoge % fuga `", Literal::String("hoge % fuga".to_owned()));
    assert_parsed("` hoge\nfuga `", Literal::String("hoge\nfuga".to_owned()));
    assert_not_parsed::<Literal>("``");
}
