use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::{many1, separated_list1},
    IResult,
};

const INPUT: &str = include_str!("input");

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    fn possible(&self, max_sets: &Vec<Set>) -> bool {
        self.subsets.iter().all(|subset| subset.possible(max_sets))
    }

    fn fewest_possible(&self) -> Subset {
        let sets = vec![Color::Red, Color::Blue, Color::Green]
            .iter()
            .map(|color| {
                self.subsets
                    .iter()
                    .filter_map(|subset| subset.sets.iter().find(|set| &set.color == color))
                    .max_by_key(|set| set.count)
                    .unwrap()
                    .clone()
            })
            .collect();

        Subset { sets }
    }
}

#[derive(Debug)]
struct Subset {
    sets: Vec<Set>,
}

impl Subset {
    fn power(&self) -> u32 {
        self.sets.iter().map(|set| set.count).product()
    }
}

impl Subset {
    fn possible(&self, max_sets: &Vec<Set>) -> bool {
        max_sets.iter().all(|max_set| {
            !self
                .sets
                .iter()
                .any(|set| set.color == max_set.color && set.count > max_set.count)
        })
    }
}

#[derive(Clone, Debug)]
struct Set {
    color: Color,
    count: u32,
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    many1(parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, subsets) = separated_list1(tag(";"), parse_subset)(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            subsets,
        },
    ))
}

fn parse_subset(input: &str) -> IResult<&str, Subset> {
    let (input, sets) = separated_list1(tag(","), parse_set)(input)?;
    Ok((input, Subset { sets }))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (input, _) = tag(" ")(input)?;
    let (input, count) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((tag("red"), tag("blue"), tag("green")))(input)?;
    Ok((
        input,
        Set {
            count: count.parse().unwrap(),
            color: match color {
                "red" => Color::Red,
                "blue" => Color::Blue,
                "green" => Color::Green,
                _ => panic!("Unknown color"),
            },
        },
    ))
}

fn process_1(input: &str) -> u32 {
    let games = parse(input).unwrap().1;
    games
        .iter()
        .filter(|g| {
            Game::possible(
                g,
                &vec![
                    Set {
                        color: Color::Red,
                        count: 12,
                    },
                    Set {
                        color: Color::Green,
                        count: 13,
                    },
                    Set {
                        color: Color::Blue,
                        count: 14,
                    },
                ],
            )
        })
        .map(|g| g.id)
        .sum()
}

fn process_2(input: &str) -> u32 {
    let games = parse(input).unwrap().1;
    games
        .iter()
        .map(|g| g.fewest_possible().power())
        .sum()
}

fn main() {
    println!("{}", process_1(INPUT));
    println!("{}", process_2(INPUT));
}

#[test]
fn test_process_1() {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(8, process_1(INPUT))
}

#[test]
fn test_process_2() {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(2286, process_2(INPUT))
}
