use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

const INPUT: &str = include_str!("input");

struct Crabbies {
    crabbies: Vec<i32>,
}

impl Crabbies {
    fn median(&self) -> i32 {
        self.crabbies[self.crabbies.len() / 2]
    }

    fn min_fuel(&self) -> i32 {
        let median = self.median();
        self.crabbies.iter().map(|i| (median - i).abs()).sum()
    }
}

impl<'a> TryFrom<&'a str> for Crabbies {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let result: IResult<&str, Vec<i32>> =
            separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<i32>()))(s);

        let (remaining, mut crabbies) = result?;
        if remaining.len() > 0 {
            println!(
                "Warning: {} bytes remaining: '{}'",
                remaining.len(),
                remaining.replace("\n", "\\n")
            );
        }
        crabbies.sort();
        Ok(Self { crabbies })
    }
}

fn main() {
    let crabbies = Crabbies::try_from(INPUT).expect("parse input");
    println!("{:?}", crabbies.min_fuel());
}

#[cfg(test)]
const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn part_1() {
    let crabbies = Crabbies::try_from(TEST_INPUT).expect("parse input");
    assert_eq!(37, crabbies.min_fuel());
}
