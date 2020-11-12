pub mod common {

    pub use crate::{Rule, SatysfiParser};
    use pest::iterators::{Pair, Pairs};
    use pest::Parser;
    use pest::Span;

    // TODO: custom definition of Ord, PartialOrd
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Location {
        pub row: usize,
        pub col: usize,
    }

    #[macro_export]
    macro_rules! ranged {
        ($body:expr, ($r1:expr, $r2:expr), ($c1: expr, $c2: expr)) => {
            Ranged {
                start: Location { row: $r1, col: $c1 },
                end: Location { row: $r2, col: $c2 },
                body: $body,
            }
        };
        ($body:expr, ($c1: expr, $c2: expr)) => {
            Ranged {
                start: Location { row: 1, col: $c1 },
                end: Location { row: 1, col: $c2 },
                body: $body,
            }
        };
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ranged<T> {
        pub start: Location,
        pub end: Location,
        pub body: T,
    }

    impl<T> Ranged<T> {
        pub fn wrap(body: T, span: &Span) -> Self {
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

    pub trait Grammar: Sized {
        /// 何のルールに基づく文字列をパースすることができるか。
        fn rule() -> Rule;

        /// pair を読んで自身のデータ構造に格納する。
        fn parse_pair(pair: Pair<Rule>) -> Self;

        /// pair を読んで自身のデータ構造に格納し、さらに範囲の情報を付ける。
        fn parse_pair_ranged(pair: Pair<Rule>) -> Ranged<Self> {
            let span = pair.as_span();
            Ranged::wrap(Self::parse_pair(pair), &span)
        }

        /// 文字列をパースして自身のデータ構造に格納する。
        fn parse(text: &str) -> Result<Self, pest::error::Error<Rule>> {
            let mut pairs: Pairs<Rule> = SatysfiParser::parse(Self::rule(), text)?;
            let pair = pairs.next().unwrap();
            Ok(Self::parse_pair(pair))
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Type {
        Unknown,
        Int,
        Float,
        String,
        Function,
    }

    /// テスト用の関数。正しくパースされるかどうか検証する。
    pub fn assert_parsed<T: std::fmt::Debug + Grammar + PartialEq>(text: &str, expect: T) {
        let mut pairs: Pairs<Rule> = SatysfiParser::parse(T::rule(), text).unwrap();
        let vertical_pair = pairs.next().unwrap();
        let actual = T::parse_pair(vertical_pair);

        assert_eq!(actual, expect);
    }

    /// テスト用の関数。正しくパースされるかどうか検証する。
    pub fn assert_not_parsed<T: std::fmt::Debug + Grammar + PartialEq>(text: &str) {
        if SatysfiParser::parse(T::rule(), text).is_ok() {
            panic!(format!(
                r#"Text "{}" is successfully parsed by "{:?}" rule!"#,
                text,
                T::rule()
            ));
        }
    }
}

// pub mod program {
//
//     use super::common::Ranged;
//     use super::header::{Stage, Header};
//     use super::statement::Preamble;
//     use super::expr::Expr;
//
//     struct Program {
//         stage: Option<Ranged<Stage>>,
//         header: Vec<Ranged<Header>>,
//         preamble: Option<Ranged<Preamble>>,
//         body: Ranged<Expr>,
//     }
//
// }

pub mod header;

pub mod statement {

    use super::common::Ranged;

    pub struct Preamble(Vec<Ranged<Stmt>>);

    pub enum Stmt {
        LetStmt,
        LetBlockStmt,
        LetMathStmt,
        LetMutableStmt,
        ModuleStmt,
    }
}

pub mod expr {
    use super::common::Type;

    pub enum Expr {
        Match,
        BindStmt,
        CtrlFlow,
        Dyadic,
        UnaryOperator,
        VariantConstructor,
        Application,
        RecordMember,
        Unary,
    }

    pub struct Variable {
        name: String,
        t: Type,
    }
}

pub mod literal;

pub mod vertical;
