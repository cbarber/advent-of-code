const INPUT: &str = include_str!("input");

fn main() {
    let numbers = input_as_numbers();

    println!("part 1: {}", numbers.count_increases());

    println!("part 2: {}", numbers.sum_windows(3).count_increases());
}

fn input_as_numbers() -> Vec<i32> {
    INPUT
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

trait Aggregations {
    fn count_increases(&self) -> usize;
    fn sum_windows(&self, window_size: usize) -> Self;
}

impl Aggregations for Vec<i32> {
    fn count_increases(&self) -> usize {
        self.windows(2).filter(|item| item[0] < item[1]).count()
    }

    fn sum_windows(&self, window_size: usize) -> Self {
        self.windows(window_size)
            .map(|item| item.iter().sum())
            .collect::<Vec<_>>()
    }
}

#[test]
fn test_count_increases() {
    let numbers = vec![1, 2, 3, 0, 4];
    assert_eq!(3, numbers.count_increases());
}

#[test]
fn test_sum_windows() {
    let numbers = vec![1, 2, 3, 0, 4];
    assert_eq!(vec![6, 5, 7], numbers.sum_windows(3));
}

#[test]
fn test_part_1_result() {
    assert_eq!(1167, input_as_numbers().count_increases())
}

#[test]
fn test_part_2_result() {
    assert_eq!(1130, input_as_numbers().sum_windows(3).count_increases())
}
