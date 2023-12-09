use num::integer::lcm;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

const INPUT: &str = include_str!("input");

enum Direction {
    Left,
    Right,
}

struct Network {
    left: String,
    right: String,
}

struct Map {
    instructions: Vec<Direction>,
    network: HashMap<String, Network>,
}

impl Map {
    fn steps(&self, start: &str) -> u32 {
        let mut steps = 0;
        let mut current = start;
        let mut directions = self.instructions.iter().cycle();
        while !current.ends_with("Z") {
            let network = self.network.get(current).expect("network to exist");
            current = match directions.next().expect("direction to loop forever") {
                Direction::Left => &network.left,
                Direction::Right => &network.right,
            };
            steps += 1;
        }
        steps
    }

    fn ghost_steps(&self) -> u64 {
        self
            .network
            .keys()
            .into_iter()
            .filter(|k| k.ends_with("A"))
            .map(|k| self.steps(k) as u64)
            .fold(1, |acc, count| lcm(acc, count))
    }
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    terminated(
        many1(alt((
            map(complete::char('L'), |_| Direction::Left),
            map(complete::char('R'), |_| Direction::Right),
        ))),
        line_ending,
    )(input)
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    delimited(
        complete::char('('),
        map(
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            |(left, right): (&str, &str)| Network {
                left: left.to_string(),
                right: right.to_string(),
            },
        ),
        complete::char(')'),
    )(input)
}

fn parse_network_assignment(input: &str) -> IResult<&str, (String, Network)> {
    let (input, (left, network)) = terminated(
        separated_pair(alphanumeric1, tag(" = "), parse_network),
        line_ending,
    )(input)?;
    Ok((input, (left.to_string(), network)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, instructions) = parse_directions(input)?;
    let (input, _) = complete::char('\n')(input)?;
    let (input, network_assignments) = many1(parse_network_assignment)(input)?;
    Ok((
        input,
        Map {
            instructions,
            network: network_assignments.into_iter().collect(),
        },
    ))
}

fn process_1(input: &str) -> u32 {
    let map = parse_map(input).expect("map to parse").1;

    map.steps("AAA")
}

fn process_2(input: &str) -> u64 {
    let map = parse_map(input).expect("map to parse").1;

    map.ghost_steps()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1_first() {
    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)

";
    assert_eq!(2, process_1(INPUT))
}

#[test]
fn test_process_1_second() {
    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

";
    assert_eq!(6, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

";
    assert_eq!(6, process_2(INPUT))
}
