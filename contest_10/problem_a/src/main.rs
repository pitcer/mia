use std::io;
use std::io::{BufRead, Lines};

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
}

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lines();
    let problem = Line::from_lines(lines)?;
    let result = problem.solve();
    println!("{result}");
    Ok(())
}

struct Line {
    distance: i64,
    points: Vec<i64>,
}

impl Line {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let points_count: i64 = 1..=10i64.pow(5);
            let distance: i64 = 1..=10i64.pow(9)
        }
        parse_next!(lines => let points: Vec<i64> = [1..=10i64.pow(9); points_count]);
        Ok(Line { distance, points })
    }

    fn solve(self) -> i64 {
        let length = self.points.len();
        if length < 3 {
            return 0;
        }

        let mut count = 0;

        for (left_index, left) in self.points[0..length - 3 + 1].iter().enumerate() {
            let right_index = self.points[left_index..]
                .partition_point(|right| (right - left).abs() <= self.distance)
                + left_index;

            let sub_length = (right_index - left_index) as i64 - 1;
            if sub_length > 0 {
                count += (sub_length) * (sub_length - 1) / 2;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Line {
            distance: 3,
            points: vec![1, 2, 3, 4],
        };
        let actual = problem.solve();
        assert_eq!(4, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Line {
            distance: 2,
            points: vec![-3, -2, -1, 0],
        };
        let actual = problem.solve();
        assert_eq!(2, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = Line {
            distance: 19,
            points: vec![1, 10, 20, 30, 50],
        };
        let actual = problem.solve();
        assert_eq!(1, actual);
        Ok(())
    }
}
