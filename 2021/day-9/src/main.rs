use std::str::FromStr;

use ndarray::{Array2, ShapeError};

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct LavaTubes {
    map: Array2<u32>,
}

impl FromStr for LavaTubes {
    type Err = ShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let columns = lines.peek().expect("first line").len();
        let data = lines
            .flat_map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<u32>>();
        let rows = data.len() / columns;

        let map = Array2::from_shape_vec((rows, columns), data)?;

        Ok(Self { map })
    }
}

impl LavaTubes {
    fn low_points(&self) -> Vec<((usize, usize), u32)> {
        self.map
            .indexed_iter()
            .filter(|((row, col), val)| {
                self.less_than(row.wrapping_sub(1), *col, val)
                    && self.less_than(row + 1, *col, val)
                    && self.less_than(*row, col.wrapping_sub(1), val)
                    && self.less_than(*row, col + 1, val)
            })
            .map(|(point, val)| (point, *val))
            .collect()
    }

    fn risk(&self) -> u32 {
        self.low_points().iter().map(|(_, val)| 1 + val).sum()
    }

    fn less_than(&self, row: usize, col: usize, val: &u32) -> bool {
        self.map.get((row, col)).map_or(true, |v| val < v)
    }

    fn basin_sizes(&self) -> Vec<usize> {
        let mut sizes = self
            .low_points()
            .iter()
            .map(|(point, _)| self.basin_points(point, None).len())
            .collect::<Vec<usize>>();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
    }

    fn basin_points(&self, point: &(usize, usize), basin: Option<u32>) -> Vec<(usize, usize)> {
        match (basin, self.map.get(*point)) {
            (_, Some(curr)) if *curr == 9 => None,
            (Some(basin), Some(curr)) if basin < *curr => Some(*curr),
            (None, Some(curr)) => Some(*curr),
            _ => None,
        }
        .map_or(vec![], |curr| {
            let mut points = vec![*point];
            points.append(&mut self.basin_points(&(point.0.wrapping_sub(1), point.1), Some(curr)));
            points.append(&mut self.basin_points(&(point.0 + 1, point.1), Some(curr)));
            points.append(&mut self.basin_points(&(point.0, point.1.wrapping_sub(1)), Some(curr)));
            points.append(&mut self.basin_points(&(point.0, point.1 + 1), Some(curr)));
            points.sort();
            points.dedup();
            points
        })
    }
}

fn main() {
    let lava_tubes = INPUT.parse::<LavaTubes>().expect("input to parse");
    println!("{}", lava_tubes.risk());

    println!(
        "{}",
        lava_tubes
            .basin_sizes()
            .iter()
            .take(3)
            .map(|s| *s as u32)
            .product::<u32>()
    );
}

#[cfg(test)]
const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn part_1() {
    let lava_tubes = TEST_INPUT.parse::<LavaTubes>().expect("input to parse");
    assert_eq!(15, lava_tubes.risk());
}

#[test]
fn part_2() {
    let lava_tubes = TEST_INPUT.parse::<LavaTubes>().expect("input to parse");
    assert_eq!(
        1134u32,
        lava_tubes
            .basin_sizes()
            .iter()
            .take(3)
            .map(|s| *s as u32)
            .product()
    )
}
