use nom::{
    branch::alt,
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

const INPUT: &str = include_str!("input");

#[derive(Clone, Debug)]
struct Position {
    row: u32,
    col: u32,
}

impl Position {
    fn adjacent(&self, other: &Position) -> bool {
        let row_delta = (self.row as i32 - other.row as i32).abs();
        let col_delta = (self.col as i32 - other.col as i32).abs();
        row_delta <= 1 && col_delta <= 1
    }
}

#[derive(Debug)]
struct Symbol {
    #[allow(dead_code)]
    symbol: char,
    position: Position,
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    positions: Vec<Position>,
}

impl PartNumber {
    fn adjacent(&self, other: &Position) -> bool {
        self.positions.iter().any(|p| p.adjacent(other))
    }
}

#[derive(Debug)]
enum Element {
    Symbol(Symbol),
    PartNumber(PartNumber),
    Dot,
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_part_number(input: Span) -> IResult<Span, Element> {
    let (input, start) = position(input)?;
    let (input, number) = nom::character::complete::u32(input)?;
    let (input, stop) = position(input)?;

    let positions = (start.get_column()..stop.get_column())
        .into_iter()
        .map(|col| Position {
            row: stop.location_line(),
            col: col as u32,
        })
        .collect();
    Ok((input, Element::PartNumber(PartNumber { number, positions })))
}

fn parse_dot(input: Span) -> IResult<Span, Element> {
    let (input, _) = nom::character::complete::char('.')(input)?;
    Ok((input, Element::Dot))
}

fn parse_symbol(input: Span) -> IResult<Span, Element> {
    let (input, start) = position(input)?;
    let (input, symbol) = none_of(".\n\r0123456789")(input)?;

    let position = Position {
        row: start.location_line(),
        col: start.get_column() as u32,
    };

    Ok((input, Element::Symbol(Symbol { symbol, position })))
}

fn parse_element(input: Span) -> IResult<Span, Element> {
    alt((parse_part_number, parse_dot, parse_symbol))(input)
}

fn parse_row(input: Span) -> IResult<Span, Vec<Element>> {
    many1(parse_element)(input)
}

fn parse_rows(input: Span) -> IResult<Span, Vec<Element>> {
    let (input, rows) = separated_list1(line_ending, parse_row)(input)?;

    Ok((input, rows.into_iter().flatten().collect()))
}

fn process_1(input: &str) -> u32 {
    let (_, elements) = parse_rows(Span::new(input)).unwrap();

    let symbol_positions: Vec<Position> = elements
        .iter()
        .filter_map(|e| {
            if let Element::Symbol(s) = e {
                Some(s.position.clone())
            } else {
                None
            }
        })
        .collect();

    elements
        .iter()
        .filter_map(|e| {
            if let Element::PartNumber(pn) = e {
                if symbol_positions.iter().any(|s| pn.adjacent(s)) {
                    Some(pn.number)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

fn process_2(input: &str) -> u32 {
    let (_, elements) = parse_rows(Span::new(input)).unwrap();

    let parts: Vec<&PartNumber> = elements
        .iter()
        .filter_map(|e| {
            if let Element::PartNumber(pn) = e {
                Some(pn)
            } else {
                None
            }
        })
        .collect();

    elements
        .iter()
        .filter_map(|e| {
            if let Element::Symbol(s) = e {
                let adjacent: Vec<&&PartNumber> =
                    parts.iter().filter(|p| p.adjacent(&s.position)).collect();
                if adjacent.len() == 2 {
                    Some(adjacent[0].number * adjacent[1].number)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("{}", process_1(INPUT));
    println!("{}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(4361, process_1(INPUT));
}

#[test]
fn test_process_2() {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(467835, process_2(INPUT));
}
