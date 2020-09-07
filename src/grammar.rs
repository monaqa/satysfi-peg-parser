pub mod common {

    use pest::iterators::Pair;
    pub use crate::Rule;

    #[derive(Debug)]
pub struct Location {
        pub row: usize,
        pub col: usize
    }

    #[derive(Debug)]
    pub struct Ranged<T> {
        pub start: Location,
        pub end: Location,
        pub body: T
    }

    impl<T> Ranged<T> {

        pub fn wrap(body: T, pair: &Pair<Rule>) -> Self {
            let start = pair.as_span().start_pos().line_col();
            let end = pair.as_span().end_pos().line_col();

            let start = Location{ row: start.0, col: start.1 };
            let end = Location{ row: end.0, col: end.1 };

            Self { start, end, body }
        }
    }

    pub trait Grammar {

        fn parse(pair: Pair<Rule>) -> Self;

    }

}

pub mod program {

    use super::common::Ranged;
    use super::header::{Stage, Header};
    use super::statement::Preamble;
    use super::expr::Expr;

    struct Program {
        stage: Option<Ranged<Stage>>,
        header: Vec<Ranged<Header>>,
        preamble: Option<Ranged<Preamble>>,
        body: Ranged<Expr>,
    }

}

pub mod header {

    use super::common::Ranged;

    pub enum Stage {
        Stage0,
        Stage1,
        StagePersistent
    }

    pub enum Header {
        Require(Ranged<String>),
        Import(Ranged<String>),
    }

}

pub mod statement {

    use super::common::Ranged;

    pub struct Preamble (Vec<Ranged<Stmt>>);

    pub enum Stmt {
        LetStmt,
        LetBlockStmt,
        LetMathStmt,
        LetMutableStmt,
        ModuleStmt,
    }

}

pub mod expr {

    pub enum Expr {
        Match,
        BindStmt,
        CtrlFlow,
        Dyadic,
        UnaryOperator,
        VariantConstructor,
        Application,
        RecordMember,
        Unary
    }
}

pub mod constant {

    use super::common::{Ranged, Grammar, Rule};
    use std::i32;

    #[derive(Debug)]
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
                    Ranged::wrap((), &inner)
                ),
                Rule::int_const => Literal::Int({
                    let inner = inner.into_inner().next().unwrap();
                    Literal::parse_int(inner)
                }),
                rule => unreachable!(format!("unreachable rule: {:?}", rule))
            }
        }

    }

    impl Literal {

        fn parse_int(pair: pest::iterators::Pair<Rule>) -> Ranged<i32> {
            match pair.as_rule() {
                Rule::int_hex_const => {
                    let body = pair.as_str();
                    let without_prefix = body.trim_start_matches("0x")
                        .trim_start_matches("0X");
                    let z = i32::from_str_radix(without_prefix, 16).unwrap();
                    Ranged::wrap(z, &pair)
                }
                Rule::int_decimal_const => {
                    let body: i32 = pair.as_str().parse().unwrap();
                    Ranged::wrap(body, &pair)
                }
                _ => unreachable!()
            }

        }

    }


    #[derive(Debug)]
    pub struct Length {
        pub value: f64,
        pub unit: String,
    }

}
