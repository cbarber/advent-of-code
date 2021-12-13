use ndarray::Array2;
use std::str::FromStr;

const INPUT: &str = include_str!("input");

struct Octopuses {
    map: Array2<u32>,
}

impl FromStr for Octopuses {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = Array2::from_shape_vec(
            (10, 10),
            s.lines()
                .flat_map(|l| l.chars().filter_map(|c| c.to_digit(10)))
                .collect(),
        )
        .expect("parse octopuses");

        Ok(Self { map })
    }
}

impl Octopuses {
    fn step(&mut self) -> u32 {
        self.map.iter_mut().for_each(|o| *o += 1);

        let flashing = self
            .map
            .indexed_iter()
            .filter(|(_, o)| **o > 9)
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();

        let count = flashing.iter().map(|pos| self.flash(pos)).sum();

        self.map.iter_mut().filter(|o| **o > 9).for_each(|o| *o = 0);

        count
    }

    fn flash(&mut self, pos: &(usize, usize)) -> u32 {
        let (x, y) = pos;

        1 + self.increment(&(x.wrapping_sub(1), y.wrapping_sub(1)))
            + self.increment(&(x.wrapping_sub(1), *y))
            + self.increment(&(x.wrapping_sub(1), y + 1))
            + self.increment(&(*x, y.wrapping_sub(1)))
            + self.increment(&(*x, y + 1))
            + self.increment(&(x + 1, y.wrapping_sub(1)))
            + self.increment(&(x + 1, *y))
            + self.increment(&(x + 1, y + 1))
    }

    fn increment(&mut self, pos: &(usize, usize)) -> u32 {
        let (x, y) = pos;

        if let Some(octopus) = self.map.get_mut((*x, *y)) {
            if *octopus == 9 {
                *octopus += 1;
                self.flash(pos)
            } else {
                *octopus += 1;
                0
            }
        } else {
            0
        }
    }
}

fn main() {
    let mut octopuses = INPUT.parse::<Octopuses>().expect("parse octopuses");

    let flash_count = (0..100).map(|_| octopuses.step()).sum::<u32>();

    println!("{}", flash_count);
}

#[cfg(test)]
const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn part_1() {
    let mut octopuses = TEST_INPUT.parse::<Octopuses>().expect("parse octopuses");

    let flash_count = (0..100).map(|_| octopuses.step()).sum::<u32>();

    assert_eq!(1656, flash_count);
}
