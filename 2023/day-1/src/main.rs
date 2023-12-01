const INPUT: &str = include_str!("input");

fn process_1(input: &str) -> i32 {
    input.lines().map(digits_from_line).sum()
}

fn digits_from_line(line: &str) -> i32 {
    let digits: Vec<i32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();

    digits.first().map_or(0, |d| d * 10) + digits.last().unwrap_or(&0)
}

fn process_2(input: &str) -> i32 {
    input.lines().map(digits_and_words_from_line).sum()
}

fn digits_and_words_from_line(line: &str) -> i32 {
    let mut digits = vec![];
    for i in 0..line.len() {
        for (text, val) in DIGITS {
            if line[i..].starts_with(text) {
                digits.push(val);
                break;
            }
        }
    }

    digits.first().map_or(0, |d| d * 10) + digits.last().unwrap_or(&0)
}

const DIGITS: [(&str, i32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn main() {
    println!("{}", process_1(INPUT));
    println!("{}", process_2(INPUT));
}

#[test]
fn test_part_1() {
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(142, process_1(INPUT));
}

#[test]
fn test_part_2() {
    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(281, process_2(INPUT));
}
