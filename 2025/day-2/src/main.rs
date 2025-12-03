use anyhow::Result;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use nom::{
    IResult, Parser,
    bytes::tag,
    character::{complete::multispace0, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

const INPUT: &str = include_str!("input");

struct IdRange {
    start: String,
    stop: String,
}

lazy_static! {
    static ref ID1: Regex = Regex::new(r"^(\d+)\1$").unwrap();
    static ref ID2: Regex = Regex::new(r"^(\d+)\1+$").unwrap();
}

impl IdRange {
    fn get_invalid_ids(&self) -> Result<Vec<u64>> {
        let mut invalid_ids = Vec::new();

        let start = self.start.parse::<u64>()?;
        let stop = self.stop.parse::<u64>()?;

        for id in start..=stop {
            if ID1.is_match(&id.to_string()).unwrap() {
                invalid_ids.push(id);
            }
        }

        Ok(invalid_ids)
    }

    fn get_invalid_ids_2(&self) -> Result<Vec<u64>> {
        let mut invalid_ids = Vec::new();

        let start = self.start.parse::<u64>()?;
        let stop = self.stop.parse::<u64>()?;

        for id in start..=stop {
            if ID2.is_match(&id.to_string()).unwrap() {
                invalid_ids.push(id);
            }
        }

        Ok(invalid_ids)
    }
}

fn parse_ranges(input: &str) -> Result<Vec<IdRange>> {
    let (_, (_, id_ranges, _)) = (
        multispace0,
        separated_list1((tag(","), multispace0), parse_id_range),
        multispace0,
    )
        .parse(input)
        .map_err(|err| err.to_owned())?;

    Ok(id_ranges)
}

fn parse_id_range(input: &str) -> IResult<&str, IdRange> {
    map(
        separated_pair(digit1(), tag("-"), digit1()),
        |(start, stop): (&str, &str)| IdRange {
            start: start.to_string(),
            stop: stop.to_string(),
        },
    )
    .parse(input)
}

fn process_1(input: &str) -> u64 {
    let id_ranges = parse_ranges(input).unwrap();
    id_ranges
        .iter()
        .flat_map(|range| range.get_invalid_ids().unwrap())
        .sum()
}

fn process_2(input: &str) -> u64 {
    let id_ranges = parse_ranges(input).unwrap();
    id_ranges
        .iter()
        .flat_map(|range| range.get_invalid_ids_2().unwrap())
        .sum()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124
";
    assert_eq!(1227775554, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "

";
    assert_eq!(0, process_2(INPUT))
}

#[test]
fn test_regex_backreference_repeat() {
    let regex = Regex::new(r"^(\d+)\1+$").unwrap();

    assert!(regex.is_match("11").unwrap());
    assert!(regex.is_match("22").unwrap());
    assert!(regex.is_match("1010").unwrap());
    assert!(!regex.is_match("1698528").unwrap());
}

#[test]
fn text_ranges_to_invalid_ids() {
    // * `11-22` has two invalid IDs, `*11*` and `*22*`.
    assert_eq!(
        IdRange {
            start: "11".to_string(),
            stop: "22".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [11, 22]
    );
    // * `95-115` has one invalid ID, `*99*`.
    // * `998-1012` has one invalid ID, `*1010*`.
    // * `1188511880-1188511890` has one invalid ID, `*1188511885*`.
    // * `222220-222224` has one invalid ID, `*222222*`.
    // * `1698522-1698528` contains no invalid IDs.
    // * `446443-446449` has one invalid ID, `*446446*`.
    // * `38593856-38593862` has one invalid ID, `*38593859*`.
    assert_eq!(
        IdRange {
            start: "95".to_string(),
            stop: "115".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [99]
    );
    assert_eq!(
        IdRange {
            start: "998".to_string(),
            stop: "1012".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [1010]
    );
    assert_eq!(
        IdRange {
            start: "1188511880".to_string(),
            stop: "1188511890".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [1188511885]
    );
    assert_eq!(
        IdRange {
            start: "222220".to_string(),
            stop: "222224".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [222222]
    );
    assert_eq!(
        IdRange {
            start: "1698522".to_string(),
            stop: "1698528".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        []
    );
    assert_eq!(
        IdRange {
            start: "446443".to_string(),
            stop: "446449".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [446446]
    );
    assert_eq!(
        IdRange {
            start: "38593856".to_string(),
            stop: "38593862".to_string()
        }
        .get_invalid_ids()
        .unwrap(),
        [38593859]
    );
}
