extern crate core;

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
    let problem = MultiplicationTable::from_lines(lines)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct MultiplicationTable {
    size: u32,
    number: u32,
}

impl MultiplicationTable {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let size: u32 = 1..=10u32.pow(5);
            let number: u32 = 1..=10u32.pow(9)
        }

        Ok(MultiplicationTable { size, number })
    }

    fn solve(self) -> Result<usize> {
        let solutions = (1..=self.size)
            .filter(|divisor| self.number % divisor == 0 && self.number / divisor <= self.size)
            .count();

        Ok(solutions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = MultiplicationTable {
            size: 10,
            number: 5,
        };
        let actual = problem.solve()?;
        assert_eq!(2, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = MultiplicationTable {
            size: 6,
            number: 12,
        };
        let actual = problem.solve()?;
        assert_eq!(4, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = MultiplicationTable {
            size: 5,
            number: 13,
        };
        let actual = problem.solve()?;
        assert_eq!(0, actual);
        Ok(())
    }
}
