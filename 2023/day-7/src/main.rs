use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending, space1},
    combinator::{map_parser, value},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

const INPUT: &str = include_str!("input");

#[derive(Clone, Debug, PartialEq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

const JOKER: u8 = 11u8;

#[derive(Debug, PartialEq)]
struct HandWithWildcard(Hand);

impl HandWithWildcard {
    fn new(mut hand: Hand) -> Self {
        hand.cards.iter_mut().for_each(|c| {
            if *c == JOKER {
                *c = 1u8;
            }
        });
        Self(hand)
    }

    fn rank(&self) -> Kind {
        let jokers = self.0.cards.iter().filter(|c| **c == 1u8).count();

        if jokers == 0 {
            return self.0.rank();
        }

        let rank = match (jokers, self.0.rank()) {
            (_, Kind::FourOfAKind) => Kind::FiveOfAKind,
            (_, Kind::FullHouse) => Kind::FiveOfAKind,

            (1, Kind::TwoPair) => Kind::FullHouse,
            (2, Kind::TwoPair) => Kind::FourOfAKind,

            (1, Kind::ThreeOfAKind) => Kind::FourOfAKind,
            (3, Kind::ThreeOfAKind) => Kind::FourOfAKind,

            (1, Kind::TwoOfAKind) => Kind::ThreeOfAKind,
            (2, Kind::TwoOfAKind) => Kind::ThreeOfAKind,

            (1, Kind::OneAKind) => Kind::TwoOfAKind,

            (_, rank) => rank,
        };

        rank
    }
}

impl PartialOrd for HandWithWildcard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let rank_order = self.rank().partial_cmp(&other.rank());
        if rank_order == Some(std::cmp::Ordering::Equal) {
            self.0
                .cards
                .iter()
                .zip(other.0.cards.iter())
                .find_map(|(c1, c2)| {
                    let cmp = c1.partial_cmp(c2);
                    if cmp == Some(std::cmp::Ordering::Equal) {
                        None
                    } else {
                        Some(cmp)
                    }
                })
                .expect("hands to be different")
        } else {
            rank_order
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Kind {
    OneAKind,
    TwoOfAKind,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn rank(&self) -> Kind {
        let mut counts: Vec<u8> = self
            .cards
            .iter()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0u8) += 1u8;
                acc
            })
            .values()
            .map(|v| *v)
            .collect();
        counts.sort();
        match counts.as_slice() {
            [5] => Kind::FiveOfAKind,
            [1, 4] => Kind::FourOfAKind,
            [2, 3] => Kind::FullHouse,
            [1, 1, 3] => Kind::ThreeOfAKind,
            [1, 2, 2] => Kind::TwoPair,
            [1, 1, 1, 2] => Kind::TwoOfAKind,
            [1, 1, 1, 1, 1] => Kind::OneAKind,
            _ => panic!("invalid hand"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let rank_order = self.rank().partial_cmp(&other.rank());
        if rank_order == Some(std::cmp::Ordering::Equal) {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(c1, c2)| {
                    let cmp = c1.partial_cmp(c2);
                    if cmp == Some(std::cmp::Ordering::Equal) {
                        None
                    } else {
                        Some(cmp)
                    }
                })
                .expect("hands to be different")
        } else {
            rank_order
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, u8> {
    alt((
        value(10u8, tag("T")),
        value(JOKER, tag("J")),
        value(12u8, tag("Q")),
        value(13u8, tag("K")),
        value(14u8, tag("A")),
        map_parser(take(1usize), complete::u8),
    ))(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(count(parse_card, 5), space1, complete::u32)(input)?;
    Ok((input, Hand { cards, bid }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

fn process_1(input: &str) -> u32 {
    let mut hands = parse_input(input)
        .expect("input to be parsed as Hand structs")
        .1;

    hands.sort_by(|h1, h2| h1.partial_cmp(h2).expect("hands to be different"));
    hands
        .iter()
        .enumerate()
        .map(|(i, card)| (i as u32 + 1u32) * card.bid)
        .sum()
}

fn process_2(input: &str) -> u32 {
    let mut hands = parse_input(input)
        .expect("input to be parsed as Hand structs")
        .1
        .iter()
        .map(|h| HandWithWildcard::new(h.clone()))
        .collect::<Vec<_>>();

    hands.sort_by(|h1, h2| h1.partial_cmp(h2).expect("hands to be different"));
    hands
        .iter()
        .enumerate()
        .map(|(i, card)| (i as u32 + 1u32) * card.0.bid)
        .sum()
}

fn main() {
    println!("{}", process_1(INPUT));
    println!("{}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

";
    assert_eq!(6440, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

";
    assert_eq!(5905, process_2(INPUT))
}
