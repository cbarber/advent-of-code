use itertools::Itertools;
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "segment_sequence.pest"]
pub struct SegmentSequnceParser;

const INPUT: &str = include_str!("input");

mod Segments {
    pub const A: u8 = 0b0000_0001;
    pub const B: u8 = 0b0000_0010;
    pub const C: u8 = 0b0000_0100;
    pub const D: u8 = 0b0000_1000;
    pub const E: u8 = 0b0001_0000;
    pub const F: u8 = 0b0010_0000;
    pub const G: u8 = 0b0100_0000;

    pub fn as_vec() -> Vec<u8> {
        vec![A, B, C, D, E, F, G]
    }
}

const DIGITS: [u8; 10] = [
    Segments::A | Segments::B | Segments::C | Segments::E | Segments::F | Segments::G,
    Segments::C | Segments::F,
    Segments::A | Segments::C | Segments::D | Segments::E | Segments::G,
    Segments::A | Segments::C | Segments::D | Segments::F | Segments::G,
    Segments::B | Segments::C | Segments::D | Segments::F,
    Segments::A | Segments::B | Segments::D | Segments::F | Segments::G,
    Segments::A | Segments::B | Segments::D | Segments::E | Segments::F | Segments::G,
    Segments::A | Segments::C | Segments::F,
    Segments::A | Segments::B | Segments::C | Segments::D | Segments::E | Segments::F | Segments::G,
    Segments::A | Segments::B | Segments::C | Segments::D | Segments::F | Segments::G,
];

#[derive(Debug)]
struct SegmentSequence<'a> {
    patterns: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'a> SegmentSequence<'a> {
    fn decode(&self) -> Vec<u8> {
        if let Some(decode) = Segments::as_vec().iter().permutations(7).find(|p| {
            self.patterns
                .iter()
                .map(|s| {
                    s.chars()
                        .fold(0u8, |acc, c| acc | p[c as usize - 'a' as usize])
                })
                .all(|d| DIGITS.contains(&d))
        }) {
            self.output
                .iter()
                .map(|s| {
                    s.chars()
                        .fold(0u8, |acc, c| acc | decode[c as usize - 'a' as usize])
                })
                .filter_map(|d| DIGITS.iter().position(|i| i == &d))
                .map(|d| d as u8)
                .collect()
        } else {
            panic!("No decode found")
        }
    }
}

struct SegmentSequences<'a> {
    sequences: Vec<SegmentSequence<'a>>,
}

impl<'a> SegmentSequences<'a> {
    fn count(&self, digits: Vec<u8>) -> usize {
        self.sequences
            .iter()
            .flat_map(SegmentSequence::decode)
            .filter(|d| digits.contains(d))
            .count()
    }
}

impl<'a> TryFrom<&'a str> for SegmentSequences<'a> {
    type Error = pest::error::Error<Rule>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let pairs = SegmentSequnceParser::parse(Rule::sequence_list, value)?;
        let sequences = pairs
            .filter_map(|pair| match pair.as_rule() {
                Rule::sequence => {
                    let mut inner = pair.into_inner();
                    let patterns = inner
                        .next()
                        .expect("parse patterns")
                        .into_inner()
                        .map(|digit| match digit.as_rule() {
                            Rule::digit => digit.as_str(),
                            _ => unreachable!(),
                        })
                        .collect();
                    let output = inner
                        .next()
                        .expect("parse outputs")
                        .into_inner()
                        .map(|digit| match digit.as_rule() {
                            Rule::digit => digit.as_str(),
                            _ => unreachable!(),
                        })
                        .collect();
                    Some(SegmentSequence { patterns, output })
                }
                _ => None,
            })
            .collect();

        Ok(SegmentSequences { sequences })
    }
}

fn main() {
    let segment_sequences = SegmentSequences::try_from(INPUT).expect("segment sequences to parse");
    println!("{}", segment_sequences.count(vec![1, 4, 7, 8]));
}

#[cfg(test)]
const TEST_INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[test]
fn part_1() {
    let segment_sequences =
        SegmentSequences::try_from(TEST_INPUT).expect("segment sequences to parse");
    assert_eq!(26, segment_sequences.count(vec![1, 4, 7, 8]));
}
