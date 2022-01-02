use std::ops::RangeInclusive;

struct Target {
    x: RangeInclusive<i16>,
    y: RangeInclusive<i16>,
}

impl Target {
    fn max_y(&self) -> i16 {
        let n = -self.y.start() - 1;
        n * (n + 1) / 2
    }

    fn min_x_step(&self) -> i16 {
        ((*self.x.start() as f64 * 8.0).sqrt() / 2.0 - 0.5).ceil() as i16
    }

    fn max_x_step(&self) -> i16 {
        *self.x.end()
    }

    fn min_y_step(&self) -> i16 {
        *self.y.start()
    }

    fn max_y_step(&self) -> i16 {
        -self.y.start()
    }

    fn hits_target(&self, mut xs: i16, mut ys: i16) -> bool {
        let mut x = xs;
        let mut y = ys;
        loop {
            if self.x.contains(&x) && self.y.contains(&y) {
                return true;
            }
            if self.y.start() > &y || self.x.end() < &x {
                return false;
            }

            if xs > 0 {
                xs -= 1;
            }
            ys -= 1;

            x += xs;
            y += ys;
        }
    }

    fn count(&self) -> usize {
        let xs = self.min_x_step()..=self.max_x_step();
        let ys = self.min_y_step()..=self.max_y_step();

        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .filter(|(x, y)| self.hits_target(*x, *y))
            .count()
    }
}

fn main() {
    let target = Target {
        x: 287..=309,
        y: -76..=-48,
    };
    println!("{}", target.max_y());
    println!("{}", target.count());
}

#[test]
fn test_part_1() {
    let target = Target {
        x: 20..=30,
        y: -10..=-5,
    };
    assert_eq!(45, target.max_y())
}

#[test]
fn test_part_2() {
    let target = Target {
        x: 20..=30,
        y: -10..=-5,
    };
    assert_eq!(true, target.hits_target(6, 9));
    assert_eq!(112, target.count())
}
