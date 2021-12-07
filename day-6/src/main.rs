use std::str::FromStr;

use pest::Parser;

const INPUT: &str = include_str!("input");

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lanternfish.pest"]
pub struct LanternFishParser;

#[derive(Debug)]
struct Fish {
    age: u8,
}

impl Fish {
    fn step(&mut self) -> Option<Fish> {
        if self.age == 0 {
            self.age = 6;
            Some(Fish { age: 8 })
        } else {
            self.age -= 1;
            None
        }
    }
}

impl FromStr for Fish {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u8>().map(|age| Fish { age })
    }
}

#[derive(Debug)]
struct Fishies {
    fishies: Vec<Fish>,
}

impl Fishies {
    fn step(&mut self) {
        let mut baby_fishies = self.fishies.iter_mut().filter_map(Fish::step).collect();
        self.fishies.append(&mut baby_fishies);
    }

    fn count(&self) -> usize {
        self.fishies.len()
    }
}

impl FromStr for Fishies {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs =
            LanternFishParser::parse(Rule::fish_list, s).expect("parsed lantern fish input");

        let fishies = pairs
            .filter_map(|pair| match pair.as_rule() {
                Rule::fish => pair.as_str().parse::<Fish>().ok(),
                _ => None,
            })
            .collect::<Vec<Fish>>();

        Ok(Fishies { fishies })
    }
}

fn main() {
    let mut fishies = INPUT.parse::<Fishies>().unwrap();
    (0..80).for_each(|_| fishies.step());
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
