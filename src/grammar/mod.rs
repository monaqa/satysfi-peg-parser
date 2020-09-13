pub mod common {

    use pest::iterators::Pair;
    use pest::Span;
    pub use crate::Rule;

    // TODO: custom definition of Ord, PartialOrd
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Location {
        pub row: usize,
        pub col: usize
    }

    #[macro_export]
    macro_rules! ranged {
        ($body:expr, ($r1:expr, $r2:expr), ($c1: expr, $c2: expr)) => {
            Ranged{
                start: Location{ row: $r1, col: $c1 },
                end: Location{ row: $r2, col: $c2 },
                body: $body
            }
        };
        ($body:expr, ($c1: expr, $c2: expr)) => {
            Ranged{
                start: Location{ row: 1, col: $c1 },
                end: Location{ row: 1, col: $c2 },
                body: $body
            }
        };
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ranged<T> {
        pub start: Location,
        pub end: Location,
        pub body: T
    }

    impl<T> Ranged<T> {

        pub fn wrap(body: T, span: &Span) -> Self {
            let start = span.start_pos().line_col();
            let end = span.end_pos().line_col();

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

pub mod literal;

