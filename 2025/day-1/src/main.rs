use nom::{
    IResult, Parser,
    character::{
        complete::{alphanumeric1, line_ending, multispace0},
        digit1, one_of,
    },
    combinator::peek,
    multi::many1,
};

const INPUT: &str = include_str!("input");

#[derive(Debug)]
enum Operation {
    Left(i32),
    Right(i32),
}

fn process_1(input: &str) -> IResult<&str, i32> {
    let (remaining, (_, operations, _)) =
        (multispace0, many1(parse_line), multispace0).parse(input)?;

    let mut current = 50i32;
    let mut result = 0i32;
    for operation in operations {
        let delta = match operation {
            Operation::Left(count) => -count,
            Operation::Right(count) => count,
        };
        current = (current + delta) % 100;
        if current < 0 {
            current += 100
        }

        if current == 0 {
            result += 1
        }
    }

    return Ok((remaining, result));
}

fn parse_line(input: &str) -> IResult<&str, Operation> {
    let result = peek(alphanumeric1).parse(input);
    result?;
    let result = (one_of("LR"), digit1(), line_ending).parse(input);
    let (remaining, (direction, count_text, _)) = result?;

    let count = count_text.parse::<i32>().expect("expected numeric");

    match direction {
        'L' => Ok((remaining, Operation::Left(count))),
        'R' => Ok((remaining, Operation::Right(count))),
        _ => panic!("unexpected direction"),
    }
}

fn process_2(input: &str) -> IResult<&str, i32> {
    let (remaining, (_, operations, _)) =
        (multispace0, many1(parse_line), multispace0).parse(input)?;

    let mut current = 50i32;
    let mut result = 0i32;
    for operation in operations {
        let delta = match operation {
            Operation::Left(count) => -count,
            Operation::Right(count) => count,
        };

        result += count_zero_passes(current, delta);

        current = (current + delta) % 100;
        if current < 0 {
            current += 100
        }
    }

    return Ok((remaining, result));
}

fn count_zero_passes(current: i32, delta: i32) -> i32 {
    let mut result = delta.abs() / 100;

    let next = current + (delta % 100);
    if (current < 0 && next >= 0) || (current > 0 && next <= 0) || (next >= 100) || (next <= -100) {
        result += 1
    }

    return result;
}

fn main() {
    println!("part 1: {:?}", process_1(INPUT));
    println!("part 2: {:?}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82

";
    let (_, result) = process_1(INPUT).unwrap();
    assert_eq!(3i32, result)
}

#[test]
fn test_process_2() {
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82

";
    let (_, result) = process_2(INPUT).unwrap();
    assert_eq!(6i32, result)
}

#[test]
fn test_count_zero_passes() {
    // * The dial starts by pointing at `50`.
    // * The dial is rotated `L68` to point at `82`; during this rotation, it points at `0` *once*.
    assert_eq!(1, count_zero_passes(50, -68));
    // * The dial is rotated `L30` to point at `52`.
    // * The dial is rotated `R48` to point at `*0*`.
    assert_eq!(1, count_zero_passes(52, 48));
    // * The dial is rotated `L5` to point at `95`.
    // * The dial is rotated `R60` to point at `55`; during this rotation, it points at `0` *once*.
    // * The dial is rotated `L55` to point at `*0*`.
    assert_eq!(1, count_zero_passes(55, -55));
    // * The dial is rotated `L1` to point at `99`.
    assert_eq!(0, count_zero_passes(0, -1));
    // * The dial is rotated `L99` to point at `*0*`.
    assert_eq!(1, count_zero_passes(99, -99));
    // * The dial is rotated `R14` to point at `14`.
    // * The dial is rotated `L82` to point at `32`; during this rotation, it points at `0` *once*.
    assert_eq!(1, count_zero_passes(14, -82));

    assert_eq!(0, count_zero_passes(0, 0));
    assert_eq!(1, count_zero_passes(0, 100));
    assert_eq!(1, count_zero_passes(0, -100));
    assert_eq!(1, count_zero_passes(50, -100));
    assert_eq!(1, count_zero_passes(-50, 100));
    assert_eq!(1, count_zero_passes(-50, 50));
    assert_eq!(2, count_zero_passes(-50, 150));
}
