use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

const INPUT: &str = include_str!("input");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Ground {
    Ash,
    Rock,
}

type Line = Vec<Ground>;

#[derive(Debug)]
struct Pattern {
    #[allow(dead_code)]
    grid: HashMap<(usize, usize), Ground>,

    rows: Vec<Line>,
    cols: Vec<Line>,
}

impl Pattern {
    fn new(rows: Vec<Vec<Ground>>) -> Self {
        let mut grid = HashMap::new();
        let mut cols: Vec<Vec<Ground>> = Vec::new();

        for (row, row_data) in rows.iter().enumerate() {
            for (col, ground) in row_data.iter().enumerate() {
                grid.insert((row, col), ground.clone());
                if cols.len() <= col {
                    cols.resize_with(col + 1, Vec::new)
                }
                cols.get_mut(col).expect("col").push(ground.clone());
            }
        }
        Self { grid, rows, cols }
    }

    fn summarize(&self) -> usize {
        let row_centers = Self::centers(&self.rows);
        let col_centers = Self::centers(&self.cols);

        row_centers
            .iter()
            .filter_map(|center| Self::center_count(&self.rows, *center))
            .max()
            .unwrap_or(0)
            * 100
            + col_centers
                .iter()
                .filter_map(|center| Self::center_count(&self.cols, *center))
                .max()
                .unwrap_or(0)
    }

    fn centers(lines: &Vec<Line>) -> Vec<(usize, usize)> {
        lines
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &Line)>>()
            .windows(2)
            .filter_map(|window| {
                let left = window[0].1;
                let right = window[1].1;
                (left == right).then_some((window[0].0, window[1].0))
            })
            .collect()
    }

    fn center_count(lines: &Vec<Line>, center: (usize, usize)) -> Option<usize> {
        lines[..=center.0]
            .iter()
            .rev()
            .zip(lines[center.1..].iter())
            .all(|(l, r)| l == r)
            .then_some(center.0 + 1)
    }
}

fn parse_ground(input: &str) -> IResult<&str, Ground> {
    alt((
        map(complete::char('.'), |_| Ground::Ash),
        map(complete::char('#'), |_| Ground::Rock),
    ))(input)
}

fn parse_pattern_row(input: &str) -> IResult<&str, Vec<Ground>> {
    many1(parse_ground)(input)
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(separated_list1(line_ending, parse_pattern_row), |rows| {
        Pattern::new(rows)
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(line_ending, terminated(parse_pattern, line_ending))(input)
}

fn process_1(input: &str) -> usize {
    let (_input, patterns) = parse(input).expect("patterns to parse");
    patterns.iter().map(|p| p.summarize()).sum()
}

fn process_2(input: &str) -> usize {
    todo!()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

";
    assert_eq!(405, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "

";
    assert_eq!(0, process_2(INPUT))
}
