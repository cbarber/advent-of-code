use std::{collections::HashSet, fmt::Display, str::FromStr};
use text_io::scan;

const INPUT: &str = include_str!("input");

struct Thermal {
    coords: HashSet<(u16, u16)>,
    folds: Vec<Fold>,
}

enum Fold {
    X(u16),
    Y(u16),
}

impl Display for Thermal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
}

impl FromStr for Thermal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (coords, folds) = s.split_once("\n\n").expect("split coords and folds");

        let coords = coords
            .lines()
            .filter_map(|l| l.split_once(","))
            .map(|(x, y)| {
                (
                    x.parse::<u16>().expect("x parse"),
                    y.parse::<u16>().expect("y parse"),
                )
            })
            .collect::<HashSet<_>>();

        let folds = folds
            .lines()
            .filter_map(|l| {
                let axis: char;
                let value: u16;
                scan!(l.bytes() => "fold along {}={}", axis, value);
                match axis {
                    'x' => Some(Fold::X(value)),
                    'y' => Some(Fold::Y(value)),
                    _ => None,
                }
            })
            .rev()
            .collect::<Vec<_>>();

        Ok(Thermal { coords, folds })
    }
}

impl Thermal {
    fn shape(&self, fold_count: usize) -> (u16, u16) {
        let take = self.folds.len() - fold_count;
        let (x, y) = self
            .folds
            .iter()
            .enumerate()
            .fold((None, None), |mut acc, (index, f)| {
                match f {
                    Fold::X(v) => {
                        if index < take {
                            acc.0 = Some(v * 2 + 1)
                        } else if acc.0.is_none() {
                            acc.0 = Some(*v)
                        }
                    }
                    Fold::Y(v) => {
                        if index < take {
                            acc.1 = Some(v * 2 + 1)
                        } else if acc.1.is_none() {
                            acc.1 = Some(*v)
                        }
                    }
                }
                acc
            });
        (x.expect("find x shape"), y.expect("find y shape"))
    }

    fn grid_for(&self, fold_count: usize) -> Vec<((u16, u16), char)> {
        let (max_x, max_y) = self.shape(fold_count);
        let xs = 0..max_x;
        let ys = 0..max_y;
        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .map(|pos| (pos, self.get(pos, fold_count)))
            .collect::<Vec<_>>()
    }

    fn get(&self, pos: (u16, u16), fold_count: usize) -> char {
        let skip = self.folds.len() - fold_count;
        let points = self
            .folds
            .iter()
            .skip(skip)
            .fold(vec![pos], |mut acc, fold| {
                let mut unfold = acc
                    .iter()
                    .map(|(x, y)| match fold {
                        Fold::X(v) => (v * 2 - x, *y),
                        Fold::Y(v) => (*x, v * 2 - y),
                    })
                    .collect::<Vec<_>>();
                acc.append(&mut unfold);
                acc
            });
        if points.iter().any(|p| self.coords.contains(p)) {
            '#'
        } else {
            '*'
        }
    }
}

fn main() {
    let thermal = INPUT.parse::<Thermal>().expect("parse thermal");
    let grid = thermal.grid_for(1);
    println!("{}", grid.iter().filter(|(_, v)| *v == '#').count());
}

#[cfg(test)]
const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[test]
fn part_1() {
    let thermal = TEST_INPUT.parse::<Thermal>().expect("parse thermal");
    let grid = thermal.grid_for(1);
    assert_eq!(17, grid.iter().filter(|(_, v)| *v == '#').count());
}
