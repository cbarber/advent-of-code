#![feature(box_patterns)]
#![feature(int_roundings)]
use pest::Parser;
use std::fmt;

const INPUT: &str = include_str!("input");

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "snailfish.pest"]
pub struct SnailFishParser;

#[derive(Debug)]
enum Element {
    Pair(Box<Element>, Box<Element>),
    Value(u8),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Pair(left, right) => write!(f, "[{},{}]", left, right),
            Element::Value(value) => write!(f, "{}", value),
        }
    }
}

struct ExplodeResult {
    add_left_side: Option<u8>,
    add_right_side: Option<u8>,
    replace: bool,
}

impl Element {
    fn new(left: u8, right: u8) -> Self {
        Self::Pair(
            Box::new(Element::Value(left)),
            Box::new(Element::Value(right)),
        )
    }

    fn add(left: Element, right: Element) -> Element {
        let mut combined = Element::Pair(Box::new(left), Box::new(right));
        while combined.explode(0).is_some() || combined.split() {}
        combined
    }

    fn explode(&mut self, depth: u8) -> Option<ExplodeResult> {
        if depth >= 4 {
            if let Element::Pair(box Element::Value(left), box Element::Value(right)) = self {
                return Some(ExplodeResult {
                    add_left_side: Some(*left),
                    add_right_side: Some(*right),
                    replace: true,
                });
            }
        }

        match self {
            Element::Value(_) => None,
            Element::Pair(box left, box right) => {
                let mut result = left.explode(depth + 1);
                if let Some(mut result) = result.take() {
                    if result.replace {
                        *left = Element::Value(0);
                        result.replace = false;
                    }
                    if let Some(add) = result.add_right_side {
                        result.add_right_side = None;
                        right.add_to_nearest_right_side(add);
                    }
                    return Some(result);
                }

                let mut result = right.explode(depth + 1);
                if let Some(mut result) = result.take() {
                    if result.replace {
                        *right = Element::Value(0);
                        result.replace = false;
                    }
                    if let Some(add) = result.add_left_side {
                        result.add_left_side = None;
                        left.add_to_nearest_left_side(add);
                    }
                    return Some(result);
                }

                None
            }
        }
    }

    fn add_to_nearest_right_side(&mut self, add: u8) {
        match self {
            Element::Pair(box left, _) => left.add_to_nearest_right_side(add),
            Element::Value(value) => *value += add,
        }
    }

    fn add_to_nearest_left_side(&mut self, add: u8) {
        match self {
            Element::Pair(_, box right) => right.add_to_nearest_left_side(add),
            Element::Value(value) => *value += add,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Pair(box left, box right) => left.split() || right.split(),
            Element::Value(value) if *value >= 10u8 => {
                *self = Element::new(value.div_floor(2), value.div_ceil(2));
                true
            }
            Element::Value(_) => false,
        }
    }

    fn magnitude(&self) -> u16 {
        match self {
            Element::Pair(box left, box right) => left.magnitude() * 3u16 + right.magnitude() * 2u16,
            Element::Value(value) => *value as u16,
        }
    }
}

#[derive(Debug)]
struct Homework {
    pairs: Vec<Element>,
}

impl Homework {
    fn sum(self) -> Element {
        self.pairs
            .into_iter()
            .reduce(Element::add)
            .expect("summed pair")
    }
}

fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Element {
    let mut inner = pair.into_inner();
    let left = parse_element(&mut inner).into();
    let right = parse_element(&mut inner).into();
    Element::Pair(left, right)
}

fn parse_element(pairs: &mut pest::iterators::Pairs<Rule>) -> Element {
    pairs
        .find_map(|pair| match pair.as_rule() {
            Rule::pair => Some(parse_pair(pair)),
            Rule::number => pair.as_str().parse::<u8>().map(|i| Element::Value(i)).ok(),
            _ => None,
        })
        .expect("element to parse")
}

impl<'a> TryFrom<&'a str> for Homework {
    type Error = pest::error::Error<Rule>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut lines = SnailFishParser::parse(Rule::lines, value)?;
        let pairs = lines
            .next()
            .expect("root lines")
            .into_inner()
            .filter_map(|pair| match pair.as_rule() {
                Rule::pair => Some(parse_pair(pair)),
                _ => None,
            })
            .collect();

        Ok(Homework { pairs })
    }
}

fn main() {
    let homework = Homework::try_from(INPUT).unwrap();
    println!("part 1: {}", homework.sum().magnitude())
}

#[test]
fn test_sum_0() {
    let homework = Homework::try_from(
        r#"[1,1]
[2,2]
[3,3]
[4,4]
"#,
    )
    .unwrap()
    .sum();
    assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", format!("{}", homework))
}

#[test]
fn test_sum_1() {
    let homework = Homework::try_from(
        r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
"#,
    )
    .unwrap()
    .sum();
    assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", format!("{}", homework))
}

#[test]
fn test_sum_2() {
    let homework = Homework::try_from(
        r#"[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]
"#,
    )
    .unwrap()
    .sum();
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format!("{}", homework))
}

#[test]
fn test_sum_3() {
    let homework = Homework::try_from(
        r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
"#,
    )
    .unwrap()
    .sum();
    assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]", format!("{}", homework))
}

#[test]
fn test_sum_4() {
    let homework = Homework::try_from(
        r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
"#,
    )
    .unwrap()
    .sum();
    assert_eq!(
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        format!("{}", homework)
    )
}

#[test]
fn test_sum_5() {
    let homework = Homework::try_from(
        r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#,
    ).unwrap().sum();
    assert_eq!("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]", format!("{}", homework));
    assert_eq!(4140, homework.magnitude())
}

#[test]
fn test_magnitude() {
    let homework = Homework::try_from(
        r#"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"#,
    ).unwrap().sum();

    assert_eq!(3488, homework.magnitude())
}
