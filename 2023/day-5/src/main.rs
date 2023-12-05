use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

const INPUT: &str = include_str!("input");

struct Almanac {
    seeds: Vec<u64>,
    conversions: Vec<Conversion>,
}

impl Almanac {
    fn find_conversion(&self, from: &str) -> Option<&Conversion> {
        self.conversions.iter().find(|c| c.from == from)
    }

    fn lowest_location(&self) -> u64 {
        let mut source = "seed";
        let mut seeds = self.seeds.clone();

        while source != "location" {
            let conversion = self
                .find_conversion(source)
                .expect(&format!("conversion '{}' to exist", source));
            source = &conversion.to;

            conversion.convert(&mut seeds);
        }

        *seeds.iter().min().expect("find minimum seed")
    }
}

#[derive(Debug)]
struct Conversion {
    from: String,
    to: String,
    transformations: Vec<Transformation>,
}

impl Conversion {
    fn convert(&self, seeds: &mut Vec<u64>) {
        for seed in seeds.iter_mut() {
            if let Some(transformation) = self.transformations.iter().find(|transformation| transformation.source_start <= *seed
                && *seed < transformation.source_start + transformation.length as u64
            ) {
                *seed = transformation.dest_start + (*seed - transformation.source_start);
            }
        }
    }
}

#[derive(Debug)]
struct Transformation {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, complete::u64),
        line_ending,
    )(input)?;
    Ok((input, seeds))
}

fn parse_transformation(input: &str) -> IResult<&str, Transformation> {
    let (input, dest_start) = terminated(complete::u64, space1)(input)?;
    let (input, source_start) = terminated(complete::u64, space1)(input)?;
    let (input, length) = terminated(complete::u64, line_ending)(input)?;
    Ok((
        input,
        Transformation {
            source_start,
            dest_start,
            length,
        },
    ))
}

fn parse_conversion(input: &str) -> IResult<&str, Conversion> {
    let (input, (from, to)) =
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, transformations) = many1(parse_transformation)(input)?;
    Ok((
        input,
        Conversion {
            from: from.to_string(),
            to: to.to_string(),
            transformations,
        },
    ))
}

fn parse_conversions(input: &str) -> IResult<&str, Vec<Conversion>> {
    let (input, conversions) = separated_list1(line_ending, parse_conversion)(input)?;
    Ok((input, conversions))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, (seeds, conversions)) =
        separated_pair(parse_seeds, line_ending, parse_conversions)(input)?;
    Ok((input, Almanac { seeds, conversions }))
}

fn process_1(input: &str) -> u64 {
    let almanac = parse_almanac(input).expect("almanac to parse").1;
    almanac.lowest_location()
}

fn process_2(input: &str) -> u64 {
    todo!()
}

fn main() {
    println!("{}", process_1(INPUT));
    println!("{}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";
    assert_eq!(35, process_1(INPUT));
}

#[test]
fn test_process_2() {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

    assert_eq!(0, process_2(INPUT));
}
