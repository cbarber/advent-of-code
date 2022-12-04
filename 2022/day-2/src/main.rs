use std::str::FromStr;

const INPUT: &str = include_str!("input");

#[derive(Clone, Debug, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = PlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            s => Err(PlayError::ParseError(format!("Failed to parse: {}", s))),
        }
    }
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug)]
enum PlayError {
    ParseError(String),
}

impl FromStr for Play {
    type Err = PlayError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissors),
            c => Err(PlayError::ParseError(format!("Failed to parse: {}", c))),
        }
    }
}

impl Play {
    fn to_outcome(&self, opponent: &Play) -> Outcome {
        match (self, opponent) {
            (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper)
            | (Play::Rock, Play::Scissors) => Outcome::Win,

            (left, right) if left == right => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn from_outcome(&self, result: &Outcome) -> Play {
        match (self, result) {
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Rock, Outcome::Lose) => Play::Scissors,

            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Paper, Outcome::Lose) => Play::Rock,

            (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Scissors, Outcome::Lose) => Play::Paper,

            (_, Outcome::Draw) => self.clone(),
        }
    }
}

pub fn parse<'a, Lhs: FromStr, Rhs: FromStr>(input: &'a str) -> Vec<(Lhs, Rhs)>
where
    <Lhs as FromStr>::Err: std::fmt::Debug,
    <Rhs as FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            let first = Lhs::from_str(&line[0..1]).expect("lhs");
            let second = Rhs::from_str(&line[2..3]).expect("rhs");
            (first, second)
        })
        .collect()
}

fn play_part_1(game: &Vec<(Play, Play)>) -> u32 {
    game.iter()
        .map(|(opponent, me)| me.to_outcome(opponent).value() + me.value())
        .sum()
}

fn play_part_2(game: &Vec<(Play, Outcome)>) -> u32 {
    game.iter()
        .map(|(opponent, outcome)| opponent.from_outcome(outcome).value() + outcome.value())
        .sum()
}

fn main() {
    let game_1 = parse::<Play, Play>(INPUT);
    println!("part 1: {}", play_part_1(&game_1));

    let game_2 = parse::<Play, Outcome>(INPUT);
    println!("part 2: {}", play_part_2(&game_2));
}

#[test]
fn test_input_1() {
    let input = "A Y\nB X\nC Z\n";
    let game = parse::<Play, Play>(input);
    assert_eq!(15, play_part_1(&game))
}

#[test]
fn test_input_2() {
    let input = "A Y\nB X\nC Z\n";
    let game = parse::<Play, Outcome>(input);
    assert_eq!(12, play_part_2(&game))
}
