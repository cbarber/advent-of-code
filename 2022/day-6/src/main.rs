const INPUT: &str = include_str!("input");

fn main() {
    let marker = input_to_first_marker(INPUT);
    println!("part 1: {:?}", marker);
    let start = input_to_start_of_message(INPUT);
    println!("part 2: {:?}", start);
}

fn find_generic_marker(input: &str, size: usize) -> Option<usize> {
    let chars = input.chars().collect::<Vec<_>>();
    let mut iter = chars.windows(size).enumerate();

    while let Some((index, window)) = iter.next() {
        if (0..window.len() - 1).into_iter().rev().all(|i| {
            (i + 1..window.len()).all(|j| {
                let result = window[i] != window[j];
                if !result {
                    for _ in 0..i {
                        iter.next();
                    }
                }
                result
            })
        }) {
            return Some(index + size);
        }
    }

    None
}

fn input_to_first_marker(input: &str) -> Option<usize> {
    find_generic_marker(input, 4)
}

fn input_to_start_of_message(input: &str) -> Option<usize> {
    find_generic_marker(input, 14)
}

#[test]
fn test_input() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(Some(5), input_to_first_marker(input));
    assert_eq!(Some(23), input_to_start_of_message(input));

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(Some(6), input_to_first_marker(input));

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(Some(10), input_to_first_marker(input));

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(Some(11), input_to_first_marker(input));

    let input = "accadf";
    assert_eq!(Some(6), input_to_first_marker(input));
}
