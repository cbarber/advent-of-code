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

fn single_delta(left: &Line, right: &Line) -> Option<usize> {
    let mut iter = left.iter().zip(right.iter()).enumerate();

    let first = iter.find_map(|(index, (l, r))| (l != r).then_some(index));

    first.filter(|_| iter.all(|(_, (l, r))| l == r))
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Line>,
    cols: Vec<Line>,
}

impl Pattern {
    fn new(rows: Vec<Line>) -> Self {
        let mut cols: Vec<Vec<Ground>> = Vec::new();

        for (_, row_data) in rows.iter().enumerate() {
            for (col, ground) in row_data.iter().enumerate() {
                if cols.len() <= col {
                    cols.resize_with(col + 1, Vec::new)
                }
                cols.get_mut(col).expect("col").push(ground.clone());
            }
        }
        Self { rows, cols }
    }

    fn summarize(&self) -> Vec<usize> {
        let row_centers = Self::centers(&self.rows);
        let col_centers = Self::centers(&self.cols);

        row_centers
            .iter()
            .filter_map(|center| Self::center_count(&self.rows, *center))
            .map(|s| s * 100)
            .chain(
                col_centers
                    .iter()
                    .filter_map(|center| Self::center_count(&self.cols, *center))
            )
            .collect()
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

    fn summarize_smudge(&self) -> Vec<usize> {
        let smudged_summary = *self
            .summarize()
            .first()
            .expect("summary to exist before considering smudge");

        println!("starting search for smudge: {}", smudged_summary);

        println!("row deltas: {:?}", Self::single_deltas(&self.rows));
        println!("col deltas: {:?}", Self::single_deltas(&self.cols));

        let result: Vec<usize> = Self::single_deltas(&self.rows)
            .into_iter()
            .chain(Self::single_deltas(&self.cols).iter().map(|t| (t.1, t.0)))
            .flat_map(|(r, c)| {
                let mut rows = self.rows.clone();
                if rows[r][c] == Ground::Ash {
                    rows[r][c] = Ground::Rock;
                } else {
                    rows[r][c] = Ground::Ash;
                }
                let summary = Pattern::new(rows).summarize();
                println!("{} {}: {:?}", r, c, summary);

                summary
                    .into_iter()
                    .filter(|summary| *summary != smudged_summary)
            })
            .collect();

        if result.is_empty() {
            panic!("no smudge found");
        }

        println!();

        result
    }

    fn single_deltas(lines: &Vec<Line>) -> Vec<(usize, usize)> {
        (2..=lines.len())
            .step_by(2)
            .flat_map(|size| {
                lines
                    .iter()
                    .enumerate()
                    .collect::<Vec<(usize, &Line)>>()
                    .windows(size)
                    .filter_map(|w| {
                        let (left_pos, left) = w.first().expect("first");
                        let (right_pos, right) = w.last().expect("last");
                        single_delta(left, right)
                            .map(|pos| vec![(*left_pos, pos), (*right_pos, pos)])
                    })
                    .flatten()
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
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
    patterns
        .iter()
        .map(|p| *p.summarize().first().expect("summary to exist"))
        .sum()
}

fn process_2(input: &str) -> usize {
    let (_input, patterns) = parse(input).expect("patterns to parse");
    patterns
        .iter()
        .map(|p| *p.summarize_smudge().first().expect("smudge summary"))
        .sum()
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
    assert_eq!(400, process_2(INPUT))
}
