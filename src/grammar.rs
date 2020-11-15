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
    stage: Option<Ranged<Stage>>,
    header: Vec<Ranged<Header>>,
    preamble: Option<Ranged<Preamble>>,
    expr: Ranged<Expr>,
}

impl Grammar for Program {
    fn rule() -> Rule {
        Rule::program
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut inner = pair.into_inner();

        // header_stage があれば追加
        let mut stage = None;
        let next = inner.peek();
        match next {
            Some(pair) if pair.as_rule() == Rule::header_stage => {
                let pair_header_stage = inner.next().unwrap();
                stage = Some(Stage::parse_pair_ranged(pair_header_stage));
            }
            _ => {}
        }

        // header を追加
        let headers = inner.next().unwrap();
        let header = headers
            .into_inner()
            .map(Header::parse_pair_ranged)
            .collect();

        // preamble があれば追加
        let mut preamble = None;
        let next = inner.peek();
        match next {
            Some(pair) if pair.as_rule() == Rule::preamble => {
                let pair_preamble = inner.next().unwrap();
                preamble = Some(Preamble::parse_pair_ranged(pair_preamble));
            }
            _ => {}
        }

        // 最後に expr を追加
        let expr = Expr::parse_pair_ranged(inner.next().unwrap());

        Program {
            stage,
            header,
            preamble,
            expr,
        }
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
        Rule::header_stage
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let stage = pair.into_inner().next().unwrap().as_str();
        match stage {
            "0" => Stage::Stage0,
            "1" => Stage::Stage1,
            "persistent" => Stage::Persistent,
            _ => unreachable!(),
        }
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
        Rule::header
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut inner_header = pair.into_inner();
        let pair_header_kind = inner_header.next().unwrap();
        let pair_pkgname = inner_header.next().unwrap();
        let span_pkgname = pair_pkgname.as_span();
        let pkgname = pair_pkgname.as_str().to_owned();
        match pair_header_kind.as_str() {
            "require" => Header::Require(Ranged::wrap(pkgname, &span_pkgname)),
            "import" => Header::Import(Ranged::wrap(pkgname, &span_pkgname)),
            _ => unreachable!(),
        }
    }
}

/// プリアンブル部分。
#[derive(Debug, PartialEq)]
pub struct Preamble(Vec<Ranged<Statement>>);

impl Grammar for Preamble {
    fn rule() -> Rule {
        Rule::preamble
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        Preamble(
            pair.into_inner()
                .map(Statement::parse_pair_ranged)
                .collect(),
        )
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
    BindStmt {
        bind: Ranged<()>,
        body: Box<Ranged<Expr>>
    },
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
            _ => unreachable!(),
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
    Literal(Literal),
    Expr {
        modname: Option<Ranged<String>>,
        expr: Box<Ranged<Expr>>,
    },
    Variable {
        modname: Option<Ranged<String>>,
        var: Ranged<Variable>,
    },
}

impl Grammar for Unary {
    fn rule() -> Rule {
        todo!()
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::record => Unary::Record(Record::parse_pair(inner)),
            Rule::list => Unary::List(List::parse_pair(inner)),
            Rule::tuple => Unary::Tuple(Tuple::parse_pair(inner)),
            Rule::bin_operator => Unary::BinOperator(String::parse_pair(inner)),
            Rule::literal => Unary::Literal(Literal::parse_pair(inner)),
            Rule::block_text => todo!(),
            Rule::horizontal_text => todo!(),
            Rule::math_text => todo!(),
            Rule::expr => {
                Unary::Expr {
                    modname: None,
                    expr: Box::new(Expr::parse_pair_ranged(inner)),
                }
            },
            Rule::expr_with_mod => todo!(),
            Rule::modvar => todo!(),
            Rule::var => todo!(),
            _ => unreachable!()
        }
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
            }
            Rule::record_inner => {
                let map = pairs.map(RecordUnit::parse_pair).collect();
                Record::Map(map)
            }
            _ => unreachable!(),
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
        RecordUnit { key, val }
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
        Rule::var
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let var_ptn = pair.into_inner().next().unwrap();
        Variable{ name: var_ptn.as_str().to_owned() }
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
pub enum Horizontal {
    Single(HorizontalSingle),
    List(Vec<Ranged<HorizontalSingle>>),
    BulletList(Vec<Ranged<HorizontalBullet>>)
}

impl Grammar for Horizontal {
    fn rule() -> Rule {
        Rule::horizontal_mode
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct HorizontalBullet {
    indent: u32,
    body: Ranged<HorizontalSingle>
}

impl Grammar for HorizontalBullet {
    fn rule() -> Rule {
        Rule::horizontal_bullet
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct HorizontalSingle (Vec<Ranged<HorizontalToken>>);

impl Grammar for HorizontalSingle {
    fn rule() -> Rule {
        Rule::horizontal_single
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum HorizontalToken {
    Text(Ranged<String>),
    SpecialChar(Ranged<String>),
    HorizontalTextEmbedding {
        mod_name: Option<Ranged<String>>,
        name: Ranged<String>,
    },
    InlineCmd {
        name: Ranged<String>,
        args: Vec<Ranged<Expr>>,
        opts: Vec<Ranged<Expr>>,
    },
    Math(Ranged<()>),
    StringLiteral(Ranged<Literal>),
}

impl Grammar for HorizontalToken {
    fn rule() -> Rule {
        Rule::horizontal_token
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Unit,
    Bool(bool),
    String(String),
    Length(Length),
    Float(f64),
    Int(i32),
}

impl Grammar for Literal {
    fn rule() -> Rule {
        Rule::literal
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::unit_const => Literal::Unit,
            Rule::bool_const => Literal::Bool(Grammar::parse_pair(inner)),
            Rule::int_const => Literal::Int(Grammar::parse_pair(inner)),
            Rule::float_const => Literal::Float(Grammar::parse_pair(inner)),
            Rule::length_const => Literal::Length(Length::parse_pair(inner)),
            Rule::string_const => Literal::String({
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

                body.to_owned()
            }),
            rule => unreachable!(format!("invalid rule: '{:?}' in rule 'literal'", rule)),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Length {
    pub value: Ranged<f64>,
    pub unit: Ranged<String>,
}

impl Grammar for Length {
    fn rule() -> Rule {
        Rule::length_const
    }

    fn parse_pair(pair: Pair<'_>) -> Self {
        let mut pairs = pair.into_inner();
        let digit = pairs.next().unwrap();
        let unit = pairs.next().unwrap();
        Length {
            value: Grammar::parse_pair_ranged(digit),
            unit: String::parse_pair_ranged(unit),
        }
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


