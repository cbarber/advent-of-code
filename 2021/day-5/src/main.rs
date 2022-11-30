use std::collections::HashMap;

const INPUT: &str = include_str!("input");

fn range(left: u16, right: u16) -> Vec<u16> {
    if left > right {
        (right..=left).rev().collect()
    } else {
        (left..=right).collect()
    }
}

#[derive(Eq, Debug, Hash, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(input: &str) -> Option<Self> {
        input
            .split_once(",")
            .map(|(x, y)| match (x.parse::<u16>(), y.parse::<u16>()) {
                (Ok(x), Ok(y)) => Some(Self { x, y }),
                _ => None,
            })
            .flatten()
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    stop: Point,
}

impl Line {
    fn new(input: &str) -> Option<Self> {
        input
            .split_once(" -> ")
            .map(
                |(start, stop)| match (Point::new(start), Point::new(stop)) {
                    (Some(start), Some(stop)) => Some(Self { start, stop }),
                    _ => None,
                },
            )
            .flatten()
    }

    fn points(&self) -> Vec<Point> {
        let points = if self.start.x == self.stop.x {
            range(self.start.y, self.stop.y)
                .iter()
                .map(|y| Point {
                    x: self.start.x,
                    y: *y,
                })
                .collect()
        } else if self.start.y == self.stop.y {
            range(self.start.x, self.stop.x)
                .iter()
                .map(|x| Point {
                    x: *x,
                    y: self.start.y,
                })
                .collect()
        } else {
            range(self.start.x, self.stop.x)
                .iter()
                .zip(range(self.start.y, self.stop.y))
                .map(|(x, y)| Point { x: *x, y })
                .collect()
        };
        points
    }
}

struct Grid {
    points: HashMap<Point, u8>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines = input.lines().filter_map(Line::new).collect::<Vec<Line>>();
        let points =
            lines
                .iter()
                .map(Line::points)
                .flatten()
                .fold(HashMap::new(), |mut acc, point| {
                    let count = acc.entry(point).or_default();
                    *count += 1;
                    acc
                });
        Self { points }
    }

    fn count_dangerous(&self) -> usize {
        self.points.iter().filter(|(_, c)| **c > 1).count()
    }
}

fn main() {
    let grid = Grid::new(INPUT);
    println!("{}", grid.count_dangerous());
}

#[cfg(test)]
const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn test_grid() {
    let grid = Grid::new(TEST_INPUT);
    assert_eq!(12, grid.count_dangerous());
}
