use std::{collections::HashMap, str::FromStr};

use pest::Parser;

const INPUT: &str = include_str!("input");

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lanternfish.pest"]
pub struct LanternFishParser;

#[derive(Debug)]
struct Fishies {
    fishies: HashMap<i8, u64>,
}

impl Fishies {
    fn step(&mut self) {
        self.fishies = self
            .fishies
            .iter()
            .map(|(age, count)| (age - 1, *count))
            .collect();
        if let Some(count) = self.fishies.remove(&-1) {
            *self.fishies.entry(6).or_insert(0) += count;
            *self.fishies.entry(8).or_insert(0) += count;
        }
    }

    fn count(&self) -> u64 {
        self.fishies.values().sum()
    }
}

impl FromStr for Fishies {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs =
            LanternFishParser::parse(Rule::fish_list, s).expect("parsed lantern fish input");

        let fishies = pairs
            .filter_map(|pair| match pair.as_rule() {
                Rule::fish => pair.as_str().parse::<i8>().ok(),
                _ => None,
            })
            .fold(HashMap::new(), |mut acc, age| {
                *acc.entry(age).or_insert(0) += 1;
                acc
            });

        Ok(Fishies { fishies })
    }
}

fn main() {
    let mut fishies = INPUT.parse::<Fishies>().unwrap();
    (0..80).for_each(|_| fishies.step());
    println!("{}", fishies.count());
    (80..256).for_each(|_| fishies.step());
    println!("{}", fishies.count());
}

#[cfg(test)]
const TEST_INPUT: &str = "3,4,3,1,2";

#[test]
fn test_part_1() {
    let mut fishies = TEST_INPUT.parse::<Fishies>().unwrap();
    (0..18).for_each(|_| fishies.step());
    assert_eq!(26, fishies.count());
    (18..80).for_each(|_| fishies.step());
    assert_eq!(5934, fishies.count());
}

#[test]
fn test_part_2() {
    let mut fishies = TEST_INPUT.parse::<Fishies>().unwrap();
    (0..256).for_each(|_| fishies.step());
    assert_eq!(26984457539, fishies.count());
}
