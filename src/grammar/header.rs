use pest::iterators::Pair;

use super::common::{Grammar, Ranged, Rule};

#[derive(Debug, PartialEq)]
pub enum Stage {
    Stage0,
    Stage1,
    StagePersistent,
}

impl Grammar for Stage {
    fn rule() -> Rule {
        Rule::header_stage
    }

    fn parse_pair(pair: Pair<Rule>) -> Self {
        let pair_stage = pair.into_inner().next().unwrap();
        let stage_name = pair_stage.as_str();
        match stage_name {
            "0" => Stage::Stage0,
            "1" => Stage::Stage1,
            "persistent" => Stage::StagePersistent,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Header {
    Require(Ranged<String>),
    Import(Ranged<String>),
}

impl Grammar for Header {
    fn rule() -> Rule {
        Rule::header
    }

    fn parse_pair(pair: Pair<Rule>) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::{
        grammar::common::{assert_not_parsed, assert_parsed, Location},
        ranged,
    };

    use super::*;

    #[test]
    fn test_stage() {
        assert_parsed("@stage: 0\n", Stage::Stage0);
        assert_parsed("@stage:0\n", Stage::Stage0);
        assert_parsed("@stage: 1  \n", Stage::Stage1);
        assert_parsed("@stage: persistent\n", Stage::StagePersistent);
        assert_not_parsed::<Stage>("@stage: 2\n");
    }

    #[test]
    fn test_header() {
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
}
