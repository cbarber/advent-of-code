use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "chunks.pest"]
struct ChunksParser;

struct Chunks<'a> {
    errors: Vec<(pest::error::Error<Rule>, &'a str)>,
}

impl<'a> TryFrom<&'a str> for Chunks<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let errors = value
            .lines()
            .map(|line| (ChunksParser::parse(Rule::chunk_list, line), line))
            .filter_map(|(res, line)| res.err().map(|err| (err, line)))
            .collect();
        Ok(Self { errors })
    }
}

impl<'a> Chunks<'a> {
    fn error_score(&self) -> u32 {
        self.errors
            .iter()
            .filter_map(|(err, line)| match err.location {
                pest::error::InputLocation::Pos(i) => line.chars().nth(i),
                pest::error::InputLocation::Span(_) => None,
            })
            .map(|c| match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            })
            .sum()
    }

    fn incomplete(&self) -> impl Iterator<Item = &str> {
        self.errors
            .iter()
            .filter(|(err, line)| match err.location {
                pest::error::InputLocation::Pos(i) => line.chars().nth(i).is_none(),
                pest::error::InputLocation::Span(_) => false,
            })
            .map(|(_, line)| *line)
    }

    fn autocompletes(&self) -> Vec<String> {
        self.incomplete().map(Self::autocomplete).collect()
    }

    fn autocomplete(input: &str) -> String {
        let mut close = String::new();
        let mut open = String::new();

        for c in input.chars().rev() {
            match (c, close.chars().last()) {
                (']' | ')' | '}' | '>', _) => close.push(c),
                ('[', Some(l)) if l == ']' => {
                    close.pop();
                }
                ('(', Some(l)) if l == ')' => {
                    close.pop();
                }
                ('{', Some(l)) if l == '}' => {
                    close.pop();
                }
                ('<', Some(l)) if l == '>' => {
                    close.pop();
                }
                ('[', _) => open.push(']'),
                ('(', _) => open.push(')'),
                ('{', _) => open.push('}'),
                ('<', _) => open.push('>'),
                _ => {}
            }
        }
        open
    }

    fn autocomplete_score(&self) -> u64 {
        let mut scores = self
            .autocompletes()
            .iter()
            .map(|s| {
                s.chars().fold(0u64, |acc, c| {
                    acc * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                })
            })
            .collect::<Vec<u64>>();
        scores.sort();
        scores[scores.len() / 2]
    }
}

const INPUT: &str = include_str!("input");

fn main() {
    let chunks = Chunks::try_from(INPUT).expect("parsed input");
    println!("{}", chunks.error_score());

    println!("{}", chunks.autocomplete_score());
}

#[cfg(test)]
const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[test]
fn part_1() {
    let chunks = Chunks::try_from(TEST_INPUT).expect("parsed input");
    assert_eq!(26397, chunks.error_score());

    assert_eq!(288957, chunks.autocomplete_score());
}
