use scan_fmt::scan_fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
struct Bucket {
    name: char,
    stack: Vec<Crate>,
}

impl Bucket {
    fn new(name: char) -> Self {
        Self {
            name,
            stack: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
struct Crate {
    name: char,
}

#[derive(Debug)]
struct Procedure {
    count: usize,
    from: char,
    to: char,
}

impl FromStr for Procedure {
    type Err = StateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, from, to) = scan_fmt!(s, "move {d} from {} to {}", usize, char, char).unwrap();
        Ok(Procedure { count, from, to })
    }
}

struct State {
    buckets: Vec<Bucket>,
    procedures: Vec<Procedure>,
}

impl State {
    fn run_9000(&mut self) {
        self.procedures.iter().for_each(|procedure| {
            (0..procedure.count).for_each(|_| {
                // println!("{:?}", procedure);
                // println!("{:?}", &self.buckets);

                let c = self
                    .buckets
                    .iter_mut()
                    .find(|b| b.name == procedure.from)
                    .unwrap()
                    .stack
                    .pop();

                if let Some(c) = c {
                    self.buckets
                        .iter_mut()
                        .find(|b| b.name == procedure.to)
                        .unwrap()
                        .stack
                        .push(c);
                } else {
                    panic!("Failed to pop from {}", procedure.from)
                }
            })
        })
    }

    fn run_9001(&mut self) {
        self.procedures.iter().for_each(|procedure| {
            let from = self
                .buckets
                .iter_mut()
                .find(|b| b.name == procedure.from)
                .unwrap();
            let mut elements = from
                .stack
                .drain(from.stack.len() - procedure.count..)
                .collect();
            self.buckets
                .iter_mut()
                .find(|b| b.name == procedure.to)
                .unwrap()
                .stack
                .append(&mut elements);
        })
    }

    fn top(&self) -> String {
        self.buckets
            .iter()
            .map(|b| b.stack.last().unwrap().name)
            .collect()
    }
}

#[derive(Debug)]
enum StateParseError {
    MissingCrateLines,
    MissingCrate,
}

impl<'a> FromStr for State {
    type Err = StateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut crate_lines = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            crate_lines.push(line)
        }
        let bucket_line = crate_lines
            .pop()
            .ok_or(StateParseError::MissingCrateLines)?;
        let mut buckets: Vec<(usize, Bucket)> = bucket_line
            .chars()
            .enumerate()
            .filter(|(_, c)| ('1'..='9').contains(c))
            .map(|(i, c)| (i, Bucket::new(c)))
            .collect();
        while let Some(crate_line) = crate_lines.pop() {
            for (index, bucket) in buckets.iter_mut() {
                let name = crate_line
                    .chars()
                    .nth(*index)
                    .ok_or(StateParseError::MissingCrate)?;

                if ('A'..='Z').contains(&name) {
                    bucket.stack.push(Crate { name })
                }
            }
        }
        let buckets = buckets.into_iter().map(|(_, bucket)| bucket).collect();

        let mut procedures = vec![];
        while let Some(line) = lines.next() {
            procedures.push(Procedure::from_str(line).unwrap());
        }

        Ok(State {
            buckets,
            procedures,
        })
    }
}

fn main() {
    let mut state = State::from_str(INPUT).unwrap();
    state.run_9000();
    println!("{}", state.top());

    let mut state = State::from_str(INPUT).unwrap();
    state.run_9001();
    println!("{}", state.top());
}

#[test]
fn test_input() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let mut state = State::from_str(input).unwrap();
    state.run_9000();
    assert_eq!("CMZ", state.top())
}

#[test]
fn test_input_2() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let mut state = State::from_str(input).unwrap();
    state.run_9001();
    assert_eq!("MCD", state.top())
}
