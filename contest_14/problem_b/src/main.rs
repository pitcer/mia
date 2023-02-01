use std::io;
use std::io::{Lines, StdinLock};

macro_rules! parse_next {
    ($lines:expr => let $name:ident: $ty:ty = [$range:expr; $count:expr]) => {
        let line = $lines.next().ok_or("Missing next line")??;
        let line = line.split_whitespace().map(|entry| entry.parse::<_>());
        let $name = line.collect::<Result<$ty, _>>()?;

        debug_assert_eq!($name.len(), $count as usize);
        for item in &$name {
            debug_assert!(($range).contains(item));
        }
    };
    ($lines:expr => $(let $name:ident: $ty:ty = $range:expr);+) => {
        let line = $lines.next().ok_or("Missing next line")??;
        let mut line = line.split_whitespace();
        $(
            let $name = line.next().ok_or_else(|| format!("Missing {}", stringify!($name)))?;
            let $name = $name.parse::<$ty>()?;
            debug_assert!(($range).contains(&$name));
        )+
        debug_assert!(line.next().is_none());
    };
    ($line:ident -> $(let $name:ident: $ty:ty = $range:expr);+) => {
        let mut line = $line.split_whitespace();
        $(
            let $name = line.next().ok_or_else(|| format!("Missing {}", stringify!($name)))?;
            let $name = $name.parse::<$ty>()?;
            debug_assert!(($range).contains(&$name));
        )+
        debug_assert!(line.next().is_none());
    };
}

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let white = Rectangle::parse(&mut lines)?;
    let first_black = Rectangle::parse(&mut lines)?;
    let second_black = Rectangle::parse(&mut lines)?;
    let result = solve(white, first_black, second_black);
    if result {
        println!("YES");
    } else {
        println!("NO");
    }
    Ok(())
}

struct Rectangle {
    bottom_left: (u32, u32),
    top_right: (u32, u32),
}

impl Rectangle {
    pub fn new(bottom_left: (u32, u32), top_right: (u32, u32)) -> Self {
        Self {
            bottom_left,
            top_right,
        }
    }

    pub fn parse(lines: &mut Lines<StdinLock>) -> Result<Self> {
        parse_next! { lines =>
            let bottom_left_x: u32 = 0..=10u32.pow(6);
            let bottom_left_y: u32 = 0..=10u32.pow(6);
            let top_right_x: u32 = 0..=10u32.pow(6);
            let top_right_y: u32 = 0..=10u32.pow(6)
        }
        Ok(Self::new(
            (bottom_left_x, bottom_left_y),
            (top_right_x, top_right_y),
        ))
    }

    pub fn contains(&self, point: (u32, u32)) -> bool {
        point.0 >= self.bottom_left.0
            && point.0 <= self.top_right.0
            && point.1 >= self.bottom_left.1
            && point.1 <= self.top_right.1
    }

    pub fn scale(&mut self, multiplier: u32) {
        self.bottom_left.0 *= multiplier;
        self.bottom_left.1 *= multiplier;
        self.top_right.0 *= multiplier;
        self.top_right.1 *= multiplier;
    }
}

fn solve(mut white: Rectangle, mut first_black: Rectangle, mut second_black: Rectangle) -> bool {
    white.scale(2);
    first_black.scale(2);
    second_black.scale(2);

    for x in (white.bottom_left.0)..=(white.top_right.0) {
        let y_top = white.top_right.1;
        let y_bottom = white.bottom_left.1;
        if (!first_black.contains((x, y_top)) && !second_black.contains((x, y_top)))
            || (!first_black.contains((x, y_bottom)) && !second_black.contains((x, y_bottom)))
        {
            return true;
        }
    }

    for y in (white.bottom_left.1)..=(white.top_right.1) {
        let x_right = white.top_right.0;
        let x_left = white.bottom_left.0;
        if (!first_black.contains((x_right, y)) && !second_black.contains((x_right, y)))
            || (!first_black.contains((x_left, y)) && !second_black.contains((x_left, y)))
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_example_1() {
        let actual = solve(
            Rectangle::new((2, 2), (4, 4)),
            Rectangle::new((1, 1), (3, 5)),
            Rectangle::new((3, 1), (5, 5)),
        );
        assert_eq!(false, actual);
    }

    #[test]
    fn test_example_2() {
        let actual = solve(
            Rectangle::new((3, 3), (7, 5)),
            Rectangle::new((0, 0), (4, 6)),
            Rectangle::new((0, 0), (7, 4)),
        );
        assert_eq!(true, actual);
    }

    #[test]
    fn test_example_3() {
        let actual = solve(
            Rectangle::new((5, 2), (10, 5)),
            Rectangle::new((3, 1), (7, 6)),
            Rectangle::new((8, 1), (11, 7)),
        );
        assert_eq!(true, actual);
    }

    #[test]
    fn test_example_4() {
        let actual = solve(
            Rectangle::new((0, 0), (1000000, 1000000)),
            Rectangle::new((0, 0), (499999, 1000000)),
            Rectangle::new((500000, 0), (1000000, 1000000)),
        );
        assert_eq!(true, actual);
    }
}
