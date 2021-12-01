fn main() {
    let input = include_bytes!("input");
    let input = String::from_utf8_lossy(input);

    let numbers = input
        .lines()
        .map(str::parse::<i32>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    let count = numbers.windows(2).filter(|item| item[0] < item[1]).count();
    println!("part 1: {}", count);

    let count = numbers
        .windows(3)
        .map(|item| item[0] + item[1] + item[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|item| item[0] < item[1])
        .count();
    println!("part 2: {}", count);
}
