const INPUT: &str = include_str!("../input");
fn main() {
    let mut elves = INPUT.lines().fold(vec![0u32], |mut acm, elm| {
        if elm.is_empty() {
            acm.push(0);
        } else if let Some(elf) = acm.last_mut() {
            let val = elm.parse::<u32>().unwrap();
            *elf += val;
        }
        acm
    });
    let max = elves.iter().max().unwrap();
    println!("{}", max);

    elves.sort();
    let sum: u32 = elves.iter().rev().take(3).sum::<u32>();
    println!("{}", sum)
}
