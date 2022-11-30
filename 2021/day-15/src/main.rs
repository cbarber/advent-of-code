use colored::Colorize;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;
use std::str::FromStr;

const INPUT: &str = include_str!("input");

struct Cave(Array2<usize>);

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.lines().nth(0).expect("first line").chars().count();
        let cave = Array2::from_shape_vec(
            (rows, cols),
            s.lines()
                .flat_map(|l| l.chars().filter_map(|c| c.to_digit(10)))
                .map(|c| c as usize)
                .collect(),
        )
        .expect("parse cave");

        Ok(Self(cave))
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn up(&self) -> Option<Pos> {
        if self.0 > 0 {
            Some(Pos(self.0 - 1, self.1))
        } else {
            None
        }
    }

    fn down(&self) -> Option<Pos> {
        Some(Pos(self.0 + 1, self.1))
    }

    fn left(&self) -> Option<Pos> {
        if self.1 > 0 {
            Some(Pos(self.0, self.1 - 1))
        } else {
            None
        }
    }

    fn right(&self) -> Option<Pos> {
        Some(Pos(self.0, self.1 + 1))
    }
}

impl Cave {
    fn neighbors(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        vec![pos.up(), pos.down(), pos.left(), pos.right()]
            .iter()
            .filter_map(|p| p.as_ref())
            .filter_map(|p| self.0.get((p.0, p.1)).map(|c| (p.clone(), *c)))
            .collect()
    }

    fn shortest_path(&self) -> Option<(Vec<Pos>, usize)> {
        let shape = self.0.shape();
        let target = Pos(shape[0] - 1, shape[1] - 1);
        dijkstra(&Pos(0, 0), |p| self.neighbors(p), |p| *p == target)
    }
}

struct FullCave(Array2<usize>);

impl FullCave {
    fn new(cave: Cave) -> Self {
        Self(cave.0)
    }

    fn get(&self, pos: &Pos) -> Option<usize> {
        let shape = self.0.shape();
        let rows = shape[0];
        let cols = shape[1];

        if pos.0 >= rows * 5 || pos.1 >= cols * 5 {
            return None;
        }

        let inc = pos.0 / rows + pos.1 / cols;
        let row = pos.0 % rows;
        let col = pos.1 % cols;

        self.0.get((row, col)).map(|c| {
            let c = c + inc;
            if c > 9 {
                (c + 1) % 10
            } else {
                c
            }
        })
    }

    fn neighbors(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        vec![pos.up(), pos.down(), pos.left(), pos.right()]
            .into_iter()
            .filter_map(|p| p)
            .filter_map(|p| self.get(&p).map(|c| (p, c)))
            .collect()
    }

    fn shortest_path(&self) -> Option<(Vec<Pos>, usize)> {
        let shape = self.0.shape();
        let rows = shape[0];
        let cols = shape[1];
        let target = Pos(rows * 5 - 1, cols * 5 - 1);
        dijkstra(&Pos(0, 0), |p| self.neighbors(p), |p| *p == target)
    }
}

fn main() {
    let cave = INPUT.parse::<Cave>().expect("parse cave");
    let result = cave.shortest_path().expect("find shortest path");
    println!("{}", result.1);

    let full_cave = FullCave::new(cave);
    let result = full_cave.shortest_path().expect("find shortest path");

    for row in 0..500 {
        for col in 0..500 {
            let out = format!("{}", full_cave.get(&Pos(row, col)).unwrap());
            if result.0.contains(&Pos(row, col)) {
                print!("{}", out.red());
            } else {
                print!("{}", out.green());
            }
        }
        println!("");
    }

    println!("{}", result.1);
}

#[cfg(test)]
const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[test]
fn part_1() {
    let cave = TEST_INPUT.parse::<Cave>().expect("parse cave");
    assert_eq!(
        Some((
            vec![
                Pos(0, 0),
                Pos(1, 0),
                Pos(2, 0),
                Pos(2, 1),
                Pos(2, 2),
                Pos(2, 3),
                Pos(2, 4),
                Pos(2, 5),
                Pos(2, 6),
                Pos(3, 6),
                Pos(3, 7),
                Pos(4, 7),
                Pos(5, 7),
                Pos(5, 8),
                Pos(6, 8),
                Pos(7, 8),
                Pos(8, 8),
                Pos(8, 9),
                Pos(9, 9)
            ],
            40
        )),
        cave.shortest_path()
    );
}

#[test]
fn part_2() {
    let cave = TEST_INPUT.parse::<Cave>().expect("parse cave");
    let full_cave = FullCave::new(cave);
    let result = full_cave.shortest_path().expect("find shortest path");

    assert_eq!(315, result.1);
}
