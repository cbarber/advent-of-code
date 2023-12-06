use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

const INPUT: &str = include_str!("input");

fn process_1(input: &str) -> u32 {
    let records = parse_records(input).unwrap().1;
    records.iter().map(|record| record.hold_count()).product()
}

fn process_2(input: &str) -> u32 {
    todo!()
}

struct Record {
    time: u32,
    distance: u32,
}

impl Record {
    fn hold_count(&self) -> u32 {
        (0..self.time)
            .into_iter()
            .filter(|time| time * (self.time - time) > self.distance)
            .count() as u32
    }
}

fn parse_records(input: &str) -> IResult<&str, Vec<Record>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) =
        delimited(space1, separated_list1(space1, complete::u32), line_ending)(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) =
        delimited(space1, separated_list1(space1, complete::u32), line_ending)(input)?;

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
    assert_eq!(0, process_2(INPUT));
}
