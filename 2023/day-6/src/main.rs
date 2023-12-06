use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

const INPUT: &str = include_str!("input");

fn process_1(input: &str) -> u64 {
    let records = parse_records(input, Kerning::Bad).unwrap().1;
    records.iter().map(|record| record.hold_count()).product()
}

fn process_2(input: &str) -> u64 {
    let records = parse_records(input, Kerning::Good).expect("input to parse into Record").1;
    records.iter().map(|record| record.hold_count()).product()
}

struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn hold_count(&self) -> u64 {
        (0..self.time)
            .into_iter()
            .filter(|time| time * (self.time - time) > self.distance)
            .count() as u64
    }
}

#[derive(Debug, PartialEq)]
enum Kerning {
    Bad,
    Good,
}

fn parse_records(input: &str, kerning: Kerning) -> IResult<&str, Vec<Record>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) = if kerning == Kerning::Bad {
        delimited(space1, separated_list1(space1, complete::u64), line_ending)(input)?
    } else {
        let (input, digits) =
            delimited(space1, separated_list1(space1, digit1), line_ending)(input)?;
        let times = vec![digits.join("").parse().expect("digits to parse into u64")];
        (input, times)
    };

    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) = if kerning == Kerning::Bad {
        delimited(space1, separated_list1(space1, complete::u64), line_ending)(input)?
    } else {
        let (input, digits) =
            delimited(space1, separated_list1(space1, digit1), line_ending)(input)?;
        let distances = vec![digits.join("").parse().expect("digits to parse into u64")];
        (input, distances)
    };

    let records = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Record {
            time: *time,
            distance: *distance,
        })
        .collect();

    Ok((input, records))
}

fn main() {
    println!("Part 1: {}", process_1(INPUT));
    println!("Part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200

";
    assert_eq!(288, process_1(INPUT));
}

#[test]
fn test_process_2() {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200

";
    assert_eq!(71503, process_2(INPUT));
}
