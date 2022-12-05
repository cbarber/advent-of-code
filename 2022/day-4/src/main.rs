use std::ops::RangeInclusive;
use include_str_in_macro::include_str_in_macro;

macro_rules! pairs {
    ($($first_min:literal-$first_max:literal,$second_min:literal-$second_max:literal)+) => {{
        vec![$(($first_min..=$first_max, $second_min..=$second_max),)+]
    }};
}

fn main() {
    let pairs = include_str_in_macro!("pairs", "day-4/src/input");
    let count = count_fully_contains(&pairs);
    println!("part 1: {}", count);

    let count = count_overlaps(&pairs);
    println!("part 2: {}", count);
}

fn contains(first: &RangeInclusive<i32>, second: &RangeInclusive<i32>) -> bool {
    (first.start() <= second.start() && first.end() >= second.end())
        || (second.start() <= first.start() && second.end() >= first.end())
}

fn count_fully_contains(pairs: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| contains(first, second))
        .count()
}

fn overlaps(first: &RangeInclusive<i32>, second: &RangeInclusive<i32>) -> bool {
    first.start() <= second.end() && second.start() <= first.end()
}

fn count_overlaps(pairs: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| overlaps(first, second))
        .count()
}

#[test]
fn test_input() {
    let pairs = pairs!(
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    );
    assert_eq!(2, count_fully_contains(&pairs));
    assert_eq!(4, count_overlaps(&pairs));
}
