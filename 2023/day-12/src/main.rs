use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

const INPUT: &str = include_str!("input");

#[derive(Clone, Debug, Eq, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct Conditions(Vec<Condition>);

impl Deref for Conditions {
    type Target = Vec<Condition>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Conditions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq)]
struct Record {
    conditions: Conditions,
    contiguous_groups: Vec<u32>,
}

impl Display for Conditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.iter() {
            match c {
                Condition::Operational => f.write_char('.'),
                Condition::Damaged => f.write_char('#'),
                Condition::Unknown => f.write_char('?'),
            }?;
        }
        Ok(())
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {:?}",
            self.conditions, self.contiguous_groups
        ))
    }
}

impl Record {
    fn matches(&self, conditions: &Conditions) -> bool {
        let mut groups: Vec<u32> = conditions
            .iter()
            .take_while(|c| c != &&Condition::Unknown)
            .fold(vec![0u32], |mut acc, c| {
                match c {
                    Condition::Operational if acc.last().expect("element") != &0 => {
                        acc.push(0);
                    }
                    Condition::Damaged => {
                        *acc.last_mut().expect("last element to exist") += 1;
                    }
                    _ => (),
                };
                acc
            })
            .into_iter()
            .filter(|g| g != &0)
            .collect();

        if groups.len() > self.contiguous_groups.len() {
            return false;
        }

        if conditions.len() == self.conditions.len() {
            return groups == self.contiguous_groups;
        }

        let tail = if conditions.len() != self.conditions.len() {
            groups
                .pop()
                .map_or(true, |last| last <= self.contiguous_groups[groups.len()])
        } else {
            true
        };

        self.contiguous_groups.starts_with(groups.as_slice()) && tail
    }

    fn possible_arragements(&self) -> usize {
        let mut current = Conditions(Vec::new());
        self.depth_first_arrangement_search(&mut current)
    }

    fn depth_first_arrangement_search(&self, current: &mut Conditions) -> usize {
        let len = current.len();
        current.extend(
            self.conditions[len..]
                .iter()
                .take_while(|c| c != &&Condition::Unknown)
                .cloned(),
        );

        let result = if !self.matches(&current) {
            0
        } else if current.len() == self.conditions.len() {
            1
        } else {
            current.push(Condition::Operational);
            let left = self.depth_first_arrangement_search(current);
            current.pop().expect("element popped");

            current.push(Condition::Damaged);
            let right = self.depth_first_arrangement_search(current);
            current.pop().expect("element popped");
            left + right
        };

        current.truncate(len);
        result
    }
}

fn parse_conditions(input: &str) -> IResult<&str, Vec<Condition>> {
    many1(alt((
        complete::char('.').map(|_| Condition::Operational),
        complete::char('#').map(|_| Condition::Damaged),
        complete::char('?').map(|_| Condition::Unknown),
    )))(input)
}

fn parse_contiguous_groups(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    map(
        separated_pair(parse_conditions, space1, parse_contiguous_groups),
        |(conditions, contiguous_groups)| Record {
            conditions: Conditions(conditions),
            contiguous_groups,
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(line_ending, parse_record)(input)
}

fn process_1(input: &str) -> usize {
    let records = parse(input).expect("records to parse").1;

    records.iter().map(|r| r.possible_arragements()).sum()
}

fn process_2(input: &str) -> u32 {
    todo!()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1

";

    assert_eq!(21, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1

";
    assert_eq!(0, process_2(INPUT))
}
