const INPUT: &str = include_str!("input");

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    vertical: i32,
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
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse(line: &str) -> Option<Command> {
    match line.split_once(" ").map(|c| (c.0, c.1.parse::<i32>().ok())) {
        Some(("forward", Some(unit))) => Some(Command::Forward(unit)),
        Some(("up", Some(unit))) => Some(Command::Up(unit)),
        Some(("down", Some(unit))) => Some(Command::Down(unit)),
        _ => None,
    }
}

fn main() {
    let commands = INPUT.lines().filter_map(parse).collect::<Vec<_>>();

    let position = commands
        .iter()
        .fold(Position::default(), |acc, c| acc.step(&c));

    println!("{:?}", position);
    println!("{}", position.horizontal * position.vertical);
}
