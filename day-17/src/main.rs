use std::ops::Range;

struct Target {
    x: Range<i16>,
    y: Range<i16>,
}

impl Target {
    fn max_y(&self) -> i16 {
        let n = -self.y.start - 1;
        n * (n + 1) / 2
    }
}

fn main() {
    let target = Target {
        x: 287..309,
        y: -76..-48,
    };
    println!("{}", target.max_y());
}

#[test]
fn test_part_1() {
    let target = Target {
        x: 20..30,
        y: -10..-5,
    };
    assert_eq!(45, target.max_y())
}
