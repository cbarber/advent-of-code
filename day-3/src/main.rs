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

fn main() {
    let diagnostics = Diagnostics::new(INPUT);
    println!("{}", diagnostics.power());
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
