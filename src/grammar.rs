//! Grammar of SATySFi.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(missing_docs)]

mod common;
mod tests;

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

    fn parse_pair(pair: Pair<'_>) -> Self {
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

    fn parse_pair(pair: Pair<'_>) -> Self {
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

    fn parse_pair(pair: Pair<'_>) -> Self {
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

    fn parse_pair(pair: Pair<'_>) -> Self {
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

    fn parse_pair(pair: Pair<'_>) -> Self {
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

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut pairs = pair.into_inner();
        let inner = pairs.next().unwrap();

        match inner.as_rule() {
            Rule::match_expr => todo!(),
            Rule::bind_stmt => {
                let sub_expr = pairs.next().unwrap();
            }
            Rule::ctrl_while => todo!(),
            Rule::ctrl_if => todo!(),
            Rule::dyadic_expr => todo!(),
            Rule::unary_operator_expr => todo!(),
            Rule::variant_constructor => todo!(),
            Rule::application => todo!(),
            Rule::record_member => todo!(),
            Rule::unary => todo!(),
            _ => unreachable!()
        }
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

    fn parse_pair(pair: Pair<'_>) -> Self {
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
        Rule::record
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut pairs = pair.into_inner();
        let pair = pairs.peek();
        if pair.is_none() {
            return Record::Map(vec![]);
        }
        match pair.unwrap().as_rule() {
            Rule::unary => {
                let pair_default = pairs.next().unwrap();
                let default = Box::new(Unary::parse_pair_ranged(pair_default));
                let map = pairs.map(RecordUnit::parse_pair).collect();
                Record::MapWithDefault { map, default }
            },
            Rule::record_inner => {
                let map = pairs.map(RecordUnit::parse_pair).collect();
                Record::Map(map)
            },
            _ => unreachable!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RecordUnit {
    key: Ranged<String>,
    val: Ranged<Expr>,
}

impl Grammar for RecordUnit {
    fn rule() -> Rule {
        Rule::record_unit
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut pairs = pair.into_inner();
        let pair_var_ptn = pairs.next().unwrap();
        let pair_expr = pairs.next().unwrap();
        let key = Ranged::wrap(pair_var_ptn.as_str().to_owned(), &pair_var_ptn.as_span());
        let val = Expr::parse_pair_ranged(pair_expr);
        RecordUnit{ key, val }
    }
}

#[derive(Debug, PartialEq)]
pub struct List(Vec<Ranged<Expr>>);

impl Grammar for List {
    fn rule() -> Rule {
        Rule::list
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let pairs_expr = pair.into_inner();
        List(pairs_expr.map(Expr::parse_pair_ranged).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct Tuple(Vec<Ranged<Expr>>);

impl Grammar for Tuple {
    fn rule() -> Rule {
        Rule::tuple
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let pairs_expr = pair.into_inner();
        Tuple(pairs_expr.map(Expr::parse_pair_ranged).collect())
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

    fn parse_pair(pair: Pair<'_>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Vertical(Vec<Ranged<VerticalElement>>);

impl Grammar for Vertical {
    fn rule() -> Rule {
        Rule::vertical_mode
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        Vertical(
            pair.into_inner()
                .map(VerticalElement::parse_pair_ranged)
                .collect(),
        )
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
        mod_name: Option<Ranged<String>>,
        name: Ranged<String>,
    },
}

impl Grammar for VerticalElement {
    fn rule() -> Rule {
        Rule::vertical_element
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let inner_vertical_element = pair.into_inner().next().unwrap();
        match inner_vertical_element.as_rule() {
            Rule::block_cmd => {
                let mut inner_cmd = inner_vertical_element.into_inner();
                let name = inner_cmd.next().unwrap();
                VerticalElement::BlockCmd {
                    name: String::parse_pair_ranged(name),
                    opts: vec![],
                    args: vec![],
                }
            }
            Rule::block_text_embedding => {
                let name = inner_vertical_element.into_inner().next().unwrap();
                match name.as_rule() {
                    Rule::var_ptn => VerticalElement::BlockTextEmbedding {
                        name: String::parse_pair_ranged(name),
                        mod_name: None,
                    },
                    Rule::modvar => {
                        let mut pairs = name.into_inner();
                        let module_name = pairs.next().unwrap();
                        let var_ptn = pairs.next().unwrap();
                        VerticalElement::BlockTextEmbedding {
                            name: String::parse_pair_ranged(var_ptn),
                            mod_name: Some(String::parse_pair_ranged(module_name)),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
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

    fn parse_pair(pair: Pair<'_>) -> Self {
        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::unit_const => Literal::Unit(Ranged::wrap((), &inner.as_span())),
            Rule::bool_const => Literal::Bool(Grammar::parse_pair_ranged(inner)),
            Rule::int_const => Literal::Int(Grammar::parse_pair_ranged(inner)),
            Rule::float_const => Literal::Float(Grammar::parse_pair_ranged(inner)),
            Rule::length_const => Literal::Length(Length::parse_pair_ranged(inner)),
            Rule::string_const => Literal::String({
                let span_string_const = inner.as_span();
                let mut pairs_string_const = inner.into_inner();
                let mut trim_start = true;
                let mut trim_end = true;
                let mut term = pairs_string_const.next().unwrap();
                if let Rule::string_omit_space_identifier = term.as_rule() {
                    // string_omit_space_identifier から始まったときは trim しない
                    trim_start = false;
                    term = pairs_string_const.next().unwrap();
                }
                if let Some(t) = pairs_string_const.next() {
                    match t.as_rule() {
                        Rule::string_omit_space_identifier => {
                            // string_omit_space_identifier で終わったときは trim しない
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

                Ranged::wrap(body.to_owned(), &span_string_const)
            }),
            rule => unreachable!(format!("invalid rule: '{:?}' in rule 'literal'", rule)),
        }
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

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut pairs = pair.into_inner();
        let digit = pairs.next().unwrap();
        let unit = pairs.next().unwrap().as_str().to_owned();
        let value: f64 = digit.as_str().parse().unwrap();
        Length { value, unit }
    }
}

impl Grammar for bool {
    fn rule() -> Rule {
        Rule::bool_const
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        match pair.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        }
    }
}

impl Grammar for f64 {
    fn rule() -> Rule {
        Rule::float_const
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        pair.as_str().parse().unwrap()
    }
}

impl Grammar for i32 {
    fn rule() -> Rule {
        Rule::int_const
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let inner_int_const = pair.into_inner().next().unwrap();
        match inner_int_const.as_rule() {
            Rule::int_hex_const => {
                let digits = inner_int_const.as_str().trim_start_matches("0x");
                i32::from_str_radix(digits, 16).unwrap()
            }
            Rule::int_decimal_const => inner_int_const.as_str().parse().unwrap(),
            _ => unreachable!(),
        }
    }
}
