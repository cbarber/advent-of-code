use std::{
    collections::HashMap,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    fn matches(&self, conditions: &Conditions) -> Option<Vec<u32>> {
        let groups: Vec<u32> = conditions
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

        if conditions.len() == self.conditions.len() {
            groups == self.contiguous_groups
        } else if groups.len() > self.contiguous_groups.len() {
            false
        } else {
            self.can_match(conditions, &groups)
        }
        .then_some(groups)
    }

    fn can_match(&self, conditions: &Conditions, groups: &Vec<u32>) -> bool {
        let head = if groups.len() > 0 {
            0..(groups.len() - 1)
        } else {
            0..0
        };
        let tail = if groups.len() > 0 {
            (groups.len() - 1)..groups.len()
        } else {
            0..0
        };
        let rest = groups.len()..;

        self.contiguous_groups.get(head.clone()) == groups.get(head)
            && self
                .contiguous_groups
                .get(tail.clone())
                .zip(groups.get(tail))
                .map_or(true, |(a, b)| b.len() == 0 || b[0] <= a[0])
            && self.contiguous_groups.get(rest).map_or(true, |r| {
                r.iter().sum::<u32>() as usize + r.iter().count()
                    <= self.conditions.len() - conditions.len() + 1
            })
    }

    fn possible_arragements(&self) -> usize {
        let mut current = Conditions(Vec::new());
        let mut cache = HashMap::new();
        self.depth_first_arrangement_search(&mut current, &mut cache)
    }

    fn depth_first_arrangement_search(
        &self,
        current: &mut Conditions,
        cache: &mut HashMap<(usize, Vec<u32>), usize>,
    ) -> usize {
        let len = current.len();
        current.extend(
            self.conditions[len..]
                .iter()
                .take_while(|c| c != &&Condition::Unknown)
                .cloned(),
        );

        let result = if let Some(groups) = self.matches(&current) {
            if let Some(cache) = cache.get(&(len, groups.clone())) {
                *cache
            } else if current.len() == self.conditions.len() {
                1
            } else {
                current.push(Condition::Operational);
                let left = self.depth_first_arrangement_search(current, cache);
                current.pop().expect("element popped");

                current.push(Condition::Damaged);
                let right = self.depth_first_arrangement_search(current, cache);
                current.pop().expect("element popped");

                if current.last().is_some_and(|l| l == &Condition::Operational) {
                    cache.insert((len, groups), left + right);
                }

                left + right
            }
        } else {
            0
        };
        current.truncate(len);
        result
    }

    fn unfold(&self, times: usize) -> Record {
        let conditions = (0..times)
            .map(|_| self.conditions.clone())
            .collect::<Vec<Vec<Condition>>>()
            .join(&Condition::Unknown);
        Record {
            conditions: Conditions(conditions),
            contiguous_groups: self.contiguous_groups.repeat(times),
        }
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

fn process_2(input: &str) -> usize {
    let records = parse(input).expect("records to parse").1;

    records
        .iter()
        .map(|r| r.unfold(5))
        .map(|r| r.possible_arragements())
        .sum()
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
    assert_eq!(525152, process_2(INPUT))
}
