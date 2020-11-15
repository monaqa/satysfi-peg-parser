//! Grammar of SATySFi.
#![allow(clippy::missing_docs_in_private_items)]

mod common;

use crate::parser::{Pair, Rule};
use common::{Grammar, Ranged};

/// プログラム全体。
#[derive(Debug, PartialEq)]
pub struct Program {
    stage: Ranged<Stage>,
    header: Vec<Ranged<Header>>,
    preamble: Ranged<Preamble>,
    expr: Ranged<()>,
}

impl Grammar for Program {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


/// どのステージに属するか。
#[derive(Debug, PartialEq)]
pub enum Stage {
    /// `@stage: 0`
    Stage0,
    /// `@stage: 1`
    Stage1,
    /// `@stage: persistent`
    Persistent,
}

impl Grammar for Stage {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


/// ヘッダ。
#[derive(Debug, PartialEq)]
pub enum Header {
    /// `@require: ...`
    Require(Ranged<String>),
    /// `@import: ...`
    Import(Ranged<String>),
}

impl Grammar for Header {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


/// プリアンブル部分。
#[derive(Debug, PartialEq)]
pub struct Preamble(Vec<Ranged<Statement>>);

impl Grammar for Preamble {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

/// let や type, module などの宣言。
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// `let ptn args = expr`
    Let {
        ptn: Ranged<()>,
        args: Vec<Ranged<()>>,
        expr: Ranged<()>,
    },
}

impl Grammar for Statement {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


/// 式。
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// `match xxx with ...`
    Match {
        expr: Box<Ranged<Expr>>,
        arms: Vec<Ranged<()>>,
    },
    /// `let xxx args = expr in`
    BindStmt,
    /// `while xxx do ...`
    CtrlFlowWhile {
        condition: Box<Ranged<Expr>>,
        body: Box<Ranged<Expr>>,
    },
    /// `if xxx then ... else ...`
    CtrlFlowIf {
        condition: Box<Ranged<Expr>>,
        expr_true: Box<Ranged<Expr>>,
        expr_false: Box<Ranged<Expr>>,
    },
    /// e.g. `1 + 2`
    Dyadic {
        lhs: Box<Ranged<Expr>>,
        rhs: Box<Ranged<Expr>>,
        binop: Ranged<String>,
    },
    /// e.g. `- 1`
    UnaryOperatorExpr {
        rhs: Box<Ranged<Expr>>,
        unaryop: Ranged<String>,
    },
    /// `Variant(a, b, c, ...)`
    VariantConstructor {
        variant: Ranged<()>,
        args: Ranged<()>,
    },
    /// `func a b ...`
    Application(Ranged<()>),
    /// `record # member`
    RecordMember,
    /// unary
    Unary(Ranged<()>),
}

impl Grammar for Expr {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


/// 単項式。
#[derive(Debug, PartialEq)]
pub enum Unary {
    BlockText(()),
    HorizontalText(()),
    MathText(()),
    Record(Record),
    List(List),
    Tuple(Tuple),
    BinOperator(String),
    Expr(Box<Expr>),
    Literal(Literal),
    ExprWithMod {
        modname: Ranged<String>,
        expr: Box<Ranged<Expr>>,
    },
    ModVar {
        modname: Ranged<String>,
        var: Ranged<Variable>,
    },
    Variable(Variable),
}

impl Grammar for Unary {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


#[derive(Debug, PartialEq)]
pub enum Record {
    Map(Vec<RecordUnit>),
    MapWithDefault {
        map: Vec<RecordUnit>,
        default: Box<Ranged<Unary>>,
    },
}

impl Grammar for Record {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


#[derive(Debug, PartialEq)]
pub struct RecordUnit {
    key: Ranged<String>,
    val: Ranged<Expr>,
}

impl Grammar for RecordUnit {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


#[derive(Debug, PartialEq)]
pub struct List(Vec<Ranged<Expr>>);

impl Grammar for List {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Tuple(Vec<Ranged<Expr>>);

impl Grammar for Tuple {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
}

impl Grammar for Variable {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}


#[derive(Debug, PartialEq)]
pub struct Vertical(Vec<Ranged<VerticalElement>>);

impl Grammar for Vertical {
    fn rule() -> Rule {
        Rule::int_const
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum VerticalElement {
    BlockCmd {
        name: Ranged<String>,
        args: Vec<Ranged<Expr>>,
        opts: Vec<Ranged<Expr>>,
    },
    BlockTextEmbedding {
        mod_name: Option<Ranged<()>>,
        name: Ranged<String>,
    },
}

impl Grammar for VerticalElement {
    fn rule() -> Rule {
        Rule::int_const
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Unit(Ranged<()>),
    Bool(Ranged<bool>),
    String(Ranged<String>),
    Length(Ranged<Length>),
    Float(Ranged<f64>),
    Int(Ranged<i32>),
}

impl Grammar for Literal {
    fn rule() -> Rule {
        Rule::literal
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        // let inner = pair.into_inner().next().unwrap();
        //
        // match inner.as_rule() {
        //     Rule::unit_const => Literal::Unit(Ranged::wrap((), &inner.as_span())),
        //     Rule::bool_const => Literal::Bool({
        //         let body = match inner.as_str() {
        //             "true" => true,
        //             "false" => false,
        //             _ => unreachable!(),
        //         };
        //         Ranged::wrap(body, &inner.as_span())
        //     }),
        //     Rule::int_const => Literal::Int({
        //         let inner = inner.into_inner().next().unwrap();
        //         Literal::parse_int(inner)
        //     }),
        //     Rule::float_const => Literal::Float({
        //         let body = inner.as_str().parse().unwrap();
        //         Ranged::wrap(body, &inner.as_span())
        //     }),
        //     Rule::length_const => Literal::Length({
        //         let span = inner.as_span();
        //         let mut pairs_inner = inner.into_inner();
        //         let digit = pairs_inner.next().unwrap();
        //         let unit = pairs_inner.next().unwrap();
        //         let value: f64 = digit.as_str().parse().unwrap();
        //         let body = Length {
        //             value,
        //             unit: unit.as_str().to_owned(),
        //         };
        //         Ranged::wrap(body, &span)
        //     }),
        //     Rule::string_const => Literal::String({
        //         let span = inner.as_span();
        //         let mut pairs_inner = inner.into_inner();
        //         let mut trim_start = true;
        //         let mut trim_end = true;
        //         let mut term = pairs_inner.next().unwrap();
        //         if let Rule::string_omit_space_identifier = term.as_rule() {
        //             trim_start = false;
        //             term = pairs_inner.next().unwrap();
        //         }
        //         if let Some(t) = pairs_inner.next() {
        //             match t.as_rule() {
        //                 Rule::string_omit_space_identifier => {
        //                     trim_end = false;
        //                 }
        //                 _ => unreachable!(),
        //             }
        //         }
        //
        //         let mut body = term.as_str();
        //         if trim_start {
        //             body = body.trim_start()
        //         }
        //         if trim_end {
        //             body = body.trim_end()
        //         }
        //
        //         Ranged::wrap(body.to_owned(), &span)
        //     }),
        //     rule => unreachable!(format!("invalid rule: '{:?}' in rule 'literal'", rule)),
        // }
        todo!()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Length {
    pub value: f64,
    pub unit: String,
}

impl Grammar for Length {
    fn rule() -> Rule {
        Rule::length_const
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

impl Grammar for bool {
    fn rule() -> Rule {
        Rule::bool_const
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

impl Grammar for f64 {
    fn rule() -> Rule {
        Rule::float_const
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}

impl Grammar for i32 {
    fn rule() -> Rule {
        Rule::int_const
    }

    fn parse_pair<'i>(pair: Pair<'i>) -> Self {
        todo!()
    }
}
