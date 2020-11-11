use super::common::{Grammar, Ranged, Rule, Type};
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub struct Vertical(Vec<Ranged<VerticalElement>>);

#[derive(Debug, PartialEq)]
pub enum VerticalElement {
    BlockCmd {
        name: String,
        t: Type,
        option: Vec<()>,
        arg: Vec<()>,
    },
    BlockTextEmbedding {
        name: String,
        t: Type,
    },
}

impl Grammar for Vertical {
    fn rule() -> Rule {
        Rule::vertical_mode
    }

    fn parse_pair(pair: Pair<Rule>) -> Self {
        // vertical_mode
        let inner = pair.into_inner();
        dbg!(&inner);
        let mut v = vec![];
        for pair in inner {
            let span = pair.as_span();
            match pair.as_rule() {
                Rule::block_cmd => {
                    let mut inner_cmd = pair.into_inner();
                    let name = inner_cmd.next().unwrap();

                    let body = VerticalElement::BlockCmd {
                        name: name.as_str().to_owned(),
                        t: Type::Unknown,
                        option: vec![],
                        arg: vec![],
                    };
                    v.push(Ranged::wrap(body, &span))
                }
                Rule::block_text_embedding => {
                    let mut inner_embedding = pair.into_inner();
                    let name = inner_embedding.next().unwrap();

                    let body = VerticalElement::BlockTextEmbedding {
                        name: name.as_str().to_owned(),
                        t: Type::Unknown,
                    };
                    v.push(Ranged::wrap(body, &span))
                }
                _ => unreachable!(),
            }
        }
        Vertical(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::common::{assert_parsed, Location};
    use crate::ranged;

    #[test]
    fn it_works() {
        assert_parsed(
            "+p{aaa}",
            Vertical(vec![ranged![
                VerticalElement::BlockCmd {
                    t: Type::Unknown,
                    name: "+p".to_owned(),
                    option: vec![],
                    arg: vec![]
                },
                (1, 8)
            ]]),
        );
    }
}
