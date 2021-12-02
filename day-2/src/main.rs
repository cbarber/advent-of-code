const INPUT: &str = include_str!("input");

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

impl Position {
    fn step(mut self, command: &Command) -> Self {
        match command {
            Command::Forward(unit) => self.horizontal += unit,
            Command::Up(unit) => self.vertical -= unit,
            Command::Down(unit) => self.vertical += unit,
        }
        self
    }

    fn aimed_step(mut self, command: &Command) -> Self {
        match command {
            Command::Forward(unit) => {
                self.horizontal += unit;
                self.vertical += self.aim * unit;
            }
            Command::Up(unit) => self.aim -= unit,
            Command::Down(unit) => self.aim += unit,
        }
        self
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse(line: &str) -> Option<Command> {
    line.split_once(" ")
        .and_then(|(text, unit)| match (text, unit.parse::<i32>()) {
            ("forward", Ok(unit)) => Some(Command::Forward(unit)),
            ("up", Ok(unit)) => Some(Command::Up(unit)),
            ("down", Ok(unit)) => Some(Command::Down(unit)),
            _ => None,
        })
}

fn main() {
    let commands = INPUT.lines().filter_map(parse).collect::<Vec<_>>();

    let position = commands
        .iter()
        .fold(Position::default(), |acc, c| acc.step(&c));
    println!("{:?}", position);
    println!("{}", position.horizontal * position.vertical);

    let position = commands
        .iter()
        .fold(Position::default(), |acc, c| acc.aimed_step(&c));
    println!("{:?}", position);
    println!("{}", position.horizontal * position.vertical);
}

#[test]
fn test_parse() {
    assert_eq!(Some(Command::Forward(1)), parse("forward 1"));
    assert_eq!(Some(Command::Up(8)), parse("up 8"));
    assert_eq!(Some(Command::Down(5)), parse("down 5"));
    assert_eq!(None, parse(""));
    assert_eq!(None, parse("garbage"));
    assert_eq!(None, parse("up8"));
}
