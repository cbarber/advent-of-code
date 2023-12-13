use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Galaxy {
    x: isize,
    y: isize,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Image {
    galaxies: Vec<Galaxy>,
    empty_x: Vec<isize>,
    empty_y: Vec<isize>,
}

impl Image {
    fn new(galaxies: Vec<Galaxy>) -> Self {
        let present_x: HashSet<isize> = galaxies.iter().map(|g| g.x).collect();
        let empty_x: Vec<isize> = (*present_x.iter().min().expect("min")
            ..*present_x.iter().max().expect("max"))
            .filter(|x| !present_x.contains(x))
            .collect();

        let present_y: HashSet<isize> = galaxies.iter().map(|g| g.y).collect();
        let empty_y: Vec<isize> = (*present_y.iter().min().expect("min")
            ..*present_y.iter().max().expect("max"))
            .filter(|y| !present_y.contains(y))
            .collect();

        let galaxies = galaxies
            .iter()
            .map(|g| Galaxy {
                x: g.x + empty_x.iter().filter(|x| **x < g.x).count() as isize,
                y: g.y + empty_y.iter().filter(|y| **y < g.y).count() as isize,
            })
            .collect();

        Self {
            galaxies,
            empty_x,
            empty_y,
        }
    }

    fn distances(&self) -> Vec<isize> {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|g| g[0].distance(&g[1]))
            .collect()
    }
}

fn parse_input(input: &str) -> Image {
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    Image::new(galaxies)
}

fn process_1(input: &str) -> isize {
    let image = parse_input(input);
    image.distances().iter().sum()
}

fn process_2(input: &str) -> u32 {
    todo!()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....

";
    assert_eq!(374, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "

";
    assert_eq!(0, process_2(INPUT))
}
