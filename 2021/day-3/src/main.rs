use std::cmp::Ordering;

const INPUT: &str = include_str!("input");

#[derive(Default)]
struct CountedBits {
    zeros: usize,
    ones: usize,
}

impl CountedBits {
    fn new(input: &Vec<&str>, index: usize) -> Self {
        input.iter().fold(CountedBits::default(), |mut acc, t| {
            if t.chars().nth(index).unwrap() == '0' {
                acc.zeros += 1
            } else {
                acc.ones += 1
            }
            acc
        })
    }
}

struct Diagnostics {
    counts: Vec<CountedBits>,
}

impl Diagnostics {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let len = lines.first().unwrap().len();

        let counts = (0..len)
            .map(|index| CountedBits::new(&lines, index))
            .collect::<Vec<_>>();

        Self { counts }
    }

    fn gamma(&self) -> u16 {
        self.aggregate(Ordering::Greater)
    }

    fn epsilon(&self) -> u16 {
        self.aggregate(Ordering::Less)
    }

    fn aggregate(&self, order: Ordering) -> u16 {
        self.counts.iter().fold(0u16, |acc, bit_counts| {
            if bit_counts.ones.cmp(&bit_counts.zeros) == order {
                (acc << 1) + 1u16
            } else {
                acc << 1
            }
        })
    }

    fn power(&self) -> u32 {
        self.gamma() as u32 * self.epsilon() as u32
    }
}

#[derive(Default)]
struct GroupedBits<'a> {
    zeros: Vec<&'a str>,
    ones: Vec<&'a str>,
}

impl<'a> GroupedBits<'a> {
    fn new(input: Vec<&'a str>, index: usize) -> Self {
        input.iter().fold(GroupedBits::default(), |mut acc, t| {
            if t.chars().nth(index).unwrap() == '0' {
                acc.zeros.push(t)
            } else {
                acc.ones.push(t)
            }
            acc
        })
    }
}

struct LifeSupport<'a> {
    lines: Vec<&'a str>,
    len: usize,
}

impl<'a> LifeSupport<'a> {
    fn new(input: &'a str) -> Self {
        let lines: Vec<&'a str> = input.lines().collect();
        let len = lines.first().unwrap().len();

        LifeSupport { lines, len }
    }

    fn oxygen(&self) -> u16 {
        self.search(|grouped| {
            if grouped.ones.len() >= grouped.zeros.len() {
                grouped.ones
            } else {
                grouped.zeros
            }
        })
    }

    fn co2(&self) -> u16 {
        self.search(|grouped| {
            if grouped.zeros.is_empty() {
                grouped.ones
            } else if grouped.ones.is_empty() {
                grouped.zeros
            } else if grouped.zeros.len() <= grouped.ones.len() {
                grouped.zeros
            } else {
                grouped.ones
            }
        })
    }

    fn rating(&self) -> u32 {
        (self.oxygen() as u32) * (self.co2() as u32)
    }

    fn search<F>(&self, mut select: F) -> u16
    where
        F: FnMut(GroupedBits<'a>) -> Vec<&'a str>,
    {
        let text = (0..self.len)
            .fold(self.lines.clone(), |acc, index| {
                select(GroupedBits::new(acc, index))
            })
            .first()
            .unwrap()
            .clone();
        u16::from_str_radix(text, 2).unwrap()
    }
}

fn main() {
    let diagnostics = Diagnostics::new(INPUT);
    println!("{}", diagnostics.power());

    let life_support = LifeSupport::new(INPUT);
    println!("{}", life_support.rating());
}

#[test]
fn test_gamma_and_epsilon() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let diagnostics = Diagnostics::new(input);
    assert_eq!(22, diagnostics.gamma());
    assert_eq!(9, diagnostics.epsilon());
    assert_eq!(198, diagnostics.power());
}

#[test]
fn test_life_support() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let life_support = LifeSupport::new(input);
    assert_eq!(23, life_support.oxygen());
    assert_eq!(10, life_support.co2());
    assert_eq!(230, life_support.rating());
}
