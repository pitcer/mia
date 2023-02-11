extern crate core;

use std::io::{BufRead, Lines};
use std::{io, vec};

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
    let problem = Badge::from_lines(lines)?;
    let result = problem.solve()?;
    result.into_iter().for_each(|result| println!("{result}"));
    Ok(())
}

struct Badge {
    reported_by: Vec<usize>,
}

impl Badge {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next!(lines => let size: usize = 1..=1000);
        parse_next!(lines => let reported_by: Vec<usize> = [1..=size; size]);
        Ok(Badge { reported_by })
    }

    fn solve(self) -> Result<Vec<usize>> {
        let solutions = (1..=self.reported_by.len())
            .map(|first| {
                let mut holes = vec![false; self.reported_by.len()];
                let mut index = first - 1;
                while !holes[index] {
                    holes[index] = true;
                    index = self.reported_by[index] - 1;
                }
                index + 1
            })
            .collect::<Vec<_>>();

        Ok(solutions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Badge {
            reported_by: vec![2, 3, 2],
        };
        let actual = problem.solve()?;
        assert_eq!(vec![2, 2, 3], actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Badge {
            reported_by: vec![1, 2, 3],
        };
        let actual = problem.solve()?;
        assert_eq!(vec![1, 2, 3], actual);
        Ok(())
    }
}
