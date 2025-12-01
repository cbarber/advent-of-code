use nom::{
    Err, IResult, Parser,
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

fn process_2(input: &str) -> i32 {
    todo!()
}

fn main() {
    println!("part 1: {:?}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
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
    const INPUT: &str = "

";
    assert_eq!(0, process_2(INPUT))
}
