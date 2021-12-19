use std::{borrow::Borrow, collections::HashMap, rc::Rc, str::FromStr};

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Polymerization {
    pairs: HashMap<PolymerPair, u64>,
    rules: HashMap<PolymerPair, char>,
    last_char: char,
}

impl FromStr for Polymerization {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pairs, rules) = s.split_once("\n\n").expect("split pairs and rules");

        let last_char = pairs.chars().last().expect("last character in pairs");

        let pairs = (0..pairs.len() - 1)
            .map(|i| (PolymerPair::new(&pairs[i..i + 2]), 1))
            .collect();

        let rules = rules
            .lines()
            .map(|l| {
                let (pair, insert) = l.split_once(" -> ").expect("split rule and insert");

                let insert = insert.chars().nth(0).expect("first character for insert");
                let pair = PolymerPair::new(pair);

                (pair, insert)
            })
            .collect();

        Ok(Self {
            pairs,
            rules,
            last_char,
        })
    }
}

impl Polymerization {
    fn step(&mut self) {
        self.pairs = self
            .pairs
            .drain()
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                let insert = self.rules.get(&pair);
                pair.expand(insert)
                    .iter()
                    .for_each(|pair| *acc.entry(pair.clone()).or_default() += count);
                acc
            })
    }

    fn min(&self) -> u64 {
        *self.counts().values().min().unwrap_or(&0)
    }

    fn max(&self) -> u64 {
        *self.counts().values().max().unwrap_or(&0)
    }

    fn counts(&self) -> HashMap<char, u64> {
        self.pairs.iter().fold(
            HashMap::from([(self.last_char, 1u64)]),
            |mut acc, (pair, count)| {
                *acc.entry(pair.first()).or_default() += count;
                acc
            },
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PolymerPair(Rc<String>);

impl Borrow<str> for PolymerPair {
    fn borrow(&self) -> &str {
        (*self.0).borrow()
    }
}

impl PolymerPair {
    fn new<S>(pair: S) -> Self
    where
        S: Into<String>,
    {
        Self(Rc::new(pair.into()))
    }

    fn first(&self) -> char {
        self.0.chars().nth(0).expect("first character")
    }

    fn expand(self, insert: Option<&char>) -> Vec<PolymerPair> {
        if let Some(insert) = insert {
            let combined = self.combined(insert);
            vec![
                PolymerPair::new(combined[0..2].to_owned()),
                PolymerPair::new(combined[1..3].to_owned()),
            ]
        } else {
            vec![self]
        }
    }

    fn combined(&self, insert: &char) -> String {
        format!("{}{}{}", &self.0[0..1], insert, &self.0[1..2])
    }
}

fn main() {
    let mut polymerization = INPUT
        .parse::<Polymerization>()
        .expect("parse polymerization");

    (0..10).for_each(|_| polymerization.step());

    println!("{}", polymerization.max() - polymerization.min());

    (10..40).for_each(|_| polymerization.step());

    println!("{}", polymerization.max() - polymerization.min());
}

#[cfg(test)]
const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

#[test]
fn test_part_1() {
    let mut polymerization = TEST_INPUT
        .parse::<Polymerization>()
        .expect("parse polymerization");

    assert_eq!(1, polymerization.min());
    assert_eq!(2, polymerization.max());

    polymerization.step();

    assert_eq!(1, polymerization.min());
    assert_eq!(2, polymerization.max());

    (1..10).for_each(|_| polymerization.step());

    assert_eq!(161, polymerization.min());
    assert_eq!(1749, polymerization.max());
}

#[test]
fn test_part_2() {
    let mut polymerization = TEST_INPUT
        .parse::<Polymerization>()
        .expect("parse polymerization");

    (0..40).for_each(|_| polymerization.step());

    assert_eq!(3849876073, polymerization.min());
    assert_eq!(2192039569602, polymerization.max());
}
