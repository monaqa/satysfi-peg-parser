//! common traits and functions.

use crate::parser::{Pair, Pairs, Rule, SatysfiParser};
use pest::Parser;
use pest::Span;

/// ソースコード上の位置を表す構造体。
// TODO: custom definition of Ord, PartialOrd
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    /// 行。1始まり。
    pub row: usize,
    /// 列。1始まり？
    pub col: usize,
}

/// ソースコード上の範囲の情報が付いた構文要素。
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ranged<T: Grammar> {
    /// 構文要素本体。
    pub body: T,
    /// body のソースコード上での開始位置。
    pub start: Location,
    /// body のソースコード上での終了位置。
    pub end: Location,
}

impl<T> Ranged<T>
where
    T: Grammar,
{
    /// body と span から新たな Ranged を作成する。
    pub fn wrap<'i>(body: T, span: &Span<'i>) -> Self {
        let start = span.start_pos().line_col();
        let end = span.end_pos().line_col();

        let start = Location {
            row: start.0,
            col: start.1,
        };
        let end = Location {
            row: end.0,
            col: end.1,
        };

        Self { start, end, body }
    }
}

/// Ranged struct を簡単に書くためのマクロ。
#[macro_export]
macro_rules! ranged {
    ($body:expr, ($r1:expr, $r2:expr), ($c1: expr, $c2: expr)) => {
        Ranged {
            start: crate::grammar::common::Location { row: $r1, col: $c1 },
            end: crate::grammar::common::Location { row: $r2, col: $c2 },
            body: $body,
        }
    };
    ($body:expr, ($c1: expr, $c2: expr)) => {
        Ranged {
            start: crate::grammar::common::Location { row: 1, col: $c1 },
            end: crate::grammar::common::Location { row: 1, col: $c2 },
            body: $body,
        }
    };
}

/// 構文要素。
pub trait Grammar: Sized {
    /// 何のルールに基づく文字列をパースすることができるか。
    fn rule() -> Rule;

    /// pair を読んで自身のデータ構造に格納する。
    fn parse_pair(pair: Pair<'_>) -> Self;

    /// pair を読んで自身のデータ構造に格納し、さらに範囲の情報を付ける。
    fn parse_pair_ranged(pair: Pair<'_>) -> Ranged<Self> {
        let span = pair.as_span();
        Ranged::wrap(Self::parse_pair(pair), &span)
    }

    /// 文字列をパースして自身のデータ構造に格納する。
    fn parse<'i>(text: &'i str) -> Result<Self, pest::error::Error<Rule>> {
        let mut pairs: Pairs<'i> = SatysfiParser::parse(Self::rule(), text)?;
        let pair = pairs.next().unwrap();
        Ok(Self::parse_pair(pair))
    }
}

/// ダミーの構文要素。実際に todo の中身が実装されることはない。
impl Grammar for () {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair(_pair: Pair<'_>) -> Self {
        todo!()
    }
}

/// ソースコードの文字列をそのまま構文要素としたい場合。
impl Grammar for String {
    /// なにかの Rule として parse することはない。
    fn rule() -> Rule {
        unreachable!()
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        pair.as_str().to_owned()
    }

    fn parse(text: &str) -> Result<Self, pest::error::Error<Rule>> {
        Ok(text.to_owned())
    }
}
