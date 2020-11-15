use super::{
    common::{Grammar, Ranged, Rule, Type},
    literal::Literal,
    vertical::Vertical,
};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Match {
        expr: Box<Ranged<Expr>>,
        arms: Vec<Ranged<MatchArm>>,
    },
    BindStmt,
    CtrlFlowWhile {
        condition: Box<Ranged<Expr>>,
        body: Box<Ranged<Expr>>,
    },
    CtrlFlowIf {
        condition: Box<Ranged<Expr>>,
        expr_true: Box<Ranged<Expr>>,
        expr_false: Box<Ranged<Expr>>,
    },
    Dyadic {
        lhs: Box<Ranged<Expr>>,
        rhs: Box<Ranged<Expr>>,
        binop: Ranged<String>,
    },
    UnaryOperatorExpr {
        rhs: Box<Ranged<Expr>>,
        unaryop: Ranged<String>,
    },
    VariantConstructor {
        variant: Ranged<Variant>,
        args: Ranged<Unary>,
    },
    Application(Ranged<Application>),
    RecordMember,
    Unary(Ranged<Unary>),
}

impl Grammar for Expr {
    fn rule() -> Rule {
        Rule::expr
    }

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        let inner = pairs.next().unwrap();

        match inner.as_rule() {
            Rule::match_expr => todo!(),
            Rule::bind_stmt => {
                let sub_expr = pairs.next().unwrap();
            }
            Rule::ctrl_flow => todo!(),
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

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    /// マッチのパターン
    ptn: (),
    /// when 句の条件
    when: Option<Ranged<Expr>>,
    /// -> の後ろ
    body: Ranged<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Application {
    General {
        var: Ranged<Variable>,
        opts: Vec<Ranged<Unary>>,
        args: Vec<Ranged<Unary>>,
    },
    Command,
}

#[derive(Debug, PartialEq)]
pub enum Unary {
    BlockText(Vertical),
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
        Rule::unary
    }

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Record{
    Map(Vec<RecordUnit>),
    MapWithDefault{
        map: Vec<RecordUnit>,
        default: Box<Ranged<Unary>>
    }
}

impl Grammar for Record {
    fn rule() -> Rule {
        Rule::record
    }

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
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

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
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

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
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

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        let pairs_expr = pair.into_inner();
        Tuple(pairs_expr.map(Expr::parse_pair_ranged).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    name: String,
    t: Type,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
    t: Type,
}

impl Grammar for Variable {
    fn rule() -> Rule {
        Rule::var
    }

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grammar::common::{assert_not_parsed, assert_parsed, Location},
        grammar::literal::Literal,
        ranged,
    };

    use super::*;

    #[test]
    fn test_tuple() {
        assert_parsed(
            "(23, 34)",
            Tuple(vec![
                // ranged![Expr::Literal(Literal::Int(ranged![23, (1, 3)])), (1, 3)],
            ]),
        );
    }
}
