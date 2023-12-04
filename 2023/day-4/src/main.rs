use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, multispace1},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

const INPUT: &str = include_str!("input");

struct Card {
    #[allow(dead_code)]
    id: u32,
    winning: Vec<u32>,
    actual: Vec<u32>,
}

impl Card {
    fn matching(&self) -> u32 {
        self.actual
            .iter()
            .filter(|&&n| self.winning.contains(&n))
            .count() as u32
    }

    fn value(&self) -> u32 {
        match self.matching() {
            0 => 0,
            shift => 2u32.pow(shift - 1),
        }
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = multispace0(input)?;
    let (input, numbers) =
        nom::multi::separated_list1(multispace1, nom::character::complete::u32)(input)?;
    Ok((input, numbers))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, (winning, actual)) =
        separated_pair(parse_numbers, tag(" | "), parse_numbers)(input)?;

    Ok((
        input,
        Card {
            id,
            winning,
            actual,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = many1(terminated(parse_card, line_ending))(input)?;
    Ok((input, cards))
}

fn process_1(input: &str) -> u32 {
    let cards = parse_cards(input).unwrap();

    cards.1.iter().map(|card| card.value()).sum()
}

#[derive(Debug)]
struct CopyTracker {
    inner: VecDeque<u32>,
}

impl CopyTracker {
    fn new(size: usize) -> Self {
        Self {
            inner: VecDeque::from(vec![1; size]),
        }
    }

    fn apply_matching(&mut self, matching: u32) -> Option<u32> {
        for i in 1..=matching {
            *self.inner.get_mut(i as usize).unwrap() += self.inner[0];
        }
        self.inner.pop_front()
    }
}

fn process_2(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).unwrap();
    let matches: Vec<u32> = cards.iter().map(|card| card.matching()).collect();

    let mut tracker = CopyTracker::new(matches.len());
    matches
        .iter()
        .fold(0u32, |acc, m| acc + tracker.apply_matching(*m).unwrap())
        .clone()
}

// 4, 2, 2, 1, 0, 0

fn main() {
    println!("Part 1: {}", process_1(INPUT));
    println!("Part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

"#;
    assert_eq!(13, process_1(INPUT));
}

#[test]
fn test_process_2() {
    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

"#;
    assert_eq!(30, process_2(INPUT));
}
