use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};
use nom_locate::{position, LocatedSpan};

const INPUT: &str = include_str!("input");

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn r#move(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Element {
    connections: Vec<Direction>,
    position: Position,
}

impl Element {
    fn complement(&self, direction: &Direction) -> Option<Direction> {
        let direction = direction.opposite();
        self.connections.iter().find_map(|d| {
            if *d != direction {
                Some(d.clone())
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
enum ParsedElement {
    Pipe(Element),
    Ground,
    Start(Position),
}

struct MapWalker<'a> {
    current: Position,
    next_direction: Direction,
    map: &'a Map,
}

impl<'a> MapWalker<'a> {
    fn new(map: &'a Map, next_direction: Direction) -> Self {
        Self {
            current: map.start.clone(),
            next_direction,
            map,
        }
    }
}

impl<'a> Iterator for MapWalker<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.map.get(&self.current, &self.next_direction) {
            self.current = next.position.clone();
            self.next_direction = next
                .complement(&self.next_direction)
                .expect("to find a complement direction");
            Some(next)
        } else {
            None
        }
    }
}

struct Map {
    start: Position,
    grid: HashMap<Position, Element>,
}

impl<'a> Map {
    fn new(elements: Vec<ParsedElement>) -> Self {
        let start: Position = elements
            .iter()
            .find_map(|e| match e {
                ParsedElement::Start(p) => Some(p.clone()),
                ParsedElement::Pipe(_) => None,
                ParsedElement::Ground => None,
            })
            .expect("to find a start position");

        let grid = elements
            .into_iter()
            .filter_map(|e| match e {
                ParsedElement::Pipe(e) => Some(e),
                ParsedElement::Ground => None,
                ParsedElement::Start(_) => None,
            })
            .fold(HashMap::new(), |mut acc, e| {
                acc.insert(e.position.clone(), e);
                acc
            });

        Self { start, grid }
    }

    fn get(&self, position: &Position, direction: &Direction) -> Option<&Element> {
        self.grid.get(&position.r#move(direction))
    }

    fn paths(&'a self) -> (MapWalker<'a>, MapWalker<'a>) {
        let mut iters: Vec<MapWalker<'a>> = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter_map(|d| {
            self.get(&self.start, d)
                .filter(|e| e.connections.contains(&d.opposite()))
                .map(|_| MapWalker::new(self, d.clone()))
        })
        .collect();

        (
            iters.pop().expect("at least one path iterator"),
            iters.pop().expect("at least two path iterators"),
        )
    }
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_element(input: Span) -> IResult<Span, ParsedElement> {
    let (input, start) = position(input)?;
    let x = start.get_column();
    let y = start.location_line() as usize;
    alt((
        map(complete::char('.'), |_| ParsedElement::Ground),
        map(complete::char('S'), move |_| {
            ParsedElement::Start(Position { x, y })
        }),
        map(
            alt((
                map(complete::char('|'), |_| {
                    vec![Direction::Up, Direction::Down]
                }),
                map(complete::char('-'), |_| {
                    vec![Direction::Left, Direction::Right]
                }),
                map(complete::char('L'), |_| {
                    vec![Direction::Up, Direction::Right]
                }),
                map(complete::char('J'), |_| {
                    vec![Direction::Up, Direction::Left]
                }),
                map(complete::char('7'), |_| {
                    vec![Direction::Down, Direction::Left]
                }),
                map(complete::char('F'), |_| {
                    vec![Direction::Down, Direction::Right]
                }),
            )),
            move |connections| {
                ParsedElement::Pipe(Element {
                    connections,
                    position: Position { x, y },
                })
            },
        ),
    ))(input)
}

fn parse_line(input: Span) -> IResult<Span, Vec<ParsedElement>> {
    terminated(many1(parse_element), line_ending)(input)
}

fn parse(input: Span) -> IResult<Span, Map> {
    let (input, lines) = many1(parse_line)(input)?;

    let elements = lines.into_iter().flatten().collect();
    Ok((input, Map::new(elements)))
}

fn process_1(input: &str) -> usize {
    let map = parse(Span::new(input)).expect("map to parse").1;
    let (forward, reverse) = map.paths();

    reverse
        .zip(forward)
        .take_while(|(f, r)| f != r)
        .count() + 1
}

fn process_2(input: &str) -> u32 {
    todo!()
}

fn main() {
    println!("part 1: {}", process_1(INPUT));
    println!("part 2: {}", process_2(INPUT));
}

#[test]
fn test_process_1_simple() {
    const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....

";
    assert_eq!(4, process_1(INPUT))
}

#[test]
fn test_process_1_simple_with_noise() {
    const INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF

";
    assert_eq!(4, process_1(INPUT))
}

#[test]
fn test_process_1_complex() {
    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...

";
    assert_eq!(8, process_1(INPUT))
}

#[test]
fn test_process_1_complex_with_noise() {
    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

";
    assert_eq!(8, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "

";
    assert_eq!(0, process_2(INPUT))
}
