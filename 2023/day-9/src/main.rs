use nom::{
    character::complete::{self, line_ending, space1},
    multi::{separated_list1, many1},
    sequence::terminated,
    IResult, combinator::map,
};

const INPUT: &str = include_str!("input");

struct History {
    elements: Vec<i64>,
}

impl History {
    fn next_value(&self) -> i64 {
        History::next(&self.elements)
    }

    fn prev_value(&self) -> i64 {
        History::prev(&self.elements)
    }

    fn next(elements: &Vec<i64>) -> i64 {
        if elements.iter().all(|e| e == &0i64)  {
            return 0i64;
        }

        let last = elements.last().expect("elements to not be empty");
        let sub = elements.windows(2).map(|w| w[1] - w[0]).collect();

        last + History::next(&sub)
    }

    fn prev(elements: &Vec<i64>) -> i64 {
        if elements.iter().all(|e| e == &0i64)  {
            return 0i64;
        }

        let first = elements.first().expect("elements to not be empty");
        let sub = elements.windows(2).map(|w| w[1] - w[0]).collect();

        first - History::prev(&sub)
    }
}

fn parse_history(input: &str) -> IResult<&str, History> {
    map(
        terminated(separated_list1(space1, complete::i64), line_ending),
        |elements| History { elements },
    )(input)
}

fn parse_histories(input: &str) -> IResult<&str, Vec<History>> {
    many1(parse_history)(input)
}

fn process_1(input: &str) -> i64 {
    let histories = parse_histories(input).expect("input to parse").1;
    histories.iter().map(|h| h.next_value()).sum()
}

fn process_2(input: &str) -> i64 {
    let histories = parse_histories(input).expect("input to parse").1;
    histories.iter().map(|h| h.prev_value()).sum()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45

";
    assert_eq!(114, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45

";
    assert_eq!(2, process_2(INPUT))
}
