const INPUT: &str = include_str!("input");

fn process(input: &str) -> i32 {
    input.lines().map(digits_from_line).sum()
}

fn digits_from_line(line: &str) -> i32 {
    let digits: Vec<i32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();

    digits.first().map_or(0, |d| d * 10) + digits.last().unwrap_or(&0)
}


fn main() {
    println!("{}", process(INPUT));
}

#[test]
fn test_part_1() {
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(142, process(INPUT));
}
