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
}
