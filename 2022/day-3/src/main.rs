#![feature(int_roundings)]
#![feature(iter_array_chunks)]

use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    let sum: u32 = INPUT.lines().filter_map(line_to_priority).sum();
    println!("part 1: {}", sum);

    let sum: u32 = INPUT
        .lines()
        .array_chunks::<3>()
        .filter_map(lines_to_badge_priority)
        .sum();
    println!("part 2: {}", sum)
}

fn char_to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("unhandled character: {}", c),
    }
}

fn line_to_priority(line: &str) -> Option<u32> {
    if line.is_empty() {
        return None;
    }

    let (one, two) = line.split_at(line.len().div_ceil(2));
    let lookup: HashSet<char> = one.chars().collect();

    two.chars()
        .find(|c| lookup.contains(c))
        .map(char_to_priority)
}

fn lines_to_badge_priority(lines: [&str; 3]) -> Option<u32> {
    let [one, two, three] = lines;

    let lookup_one: HashSet<char> = one.chars().collect();
    let lookup_two: HashSet<char> = two.chars().collect();

    three
        .chars()
        .find(|c| lookup_one.contains(c) && lookup_two.contains(c))
        .map(char_to_priority)
}

#[test]
fn test_input() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

";
    assert_eq!(157u32, input.lines().filter_map(line_to_priority).sum());

    assert_eq!(
        70u32,
        input
            .lines()
            .array_chunks::<3>()
            .filter_map(lines_to_badge_priority)
            .sum()
    );
}
