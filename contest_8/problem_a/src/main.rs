use std::io;
use std::io::{BufRead, Lines};

macro_rules! parse_next {
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
    let problem = Teams::from_lines(lines)?;
    let (minimum, maximum) = problem.solve()?;
    println!("{minimum} {maximum}");
    Ok(())
}

struct Teams {
    participants: u64,
    teams: u64,
}

impl Teams {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let participants: u64 = 1..=10u64.pow(9);
            let teams: u64 = 1..=10u64.pow(9)
        }
        debug_assert!(teams <= participants);
        Ok(Teams {
            participants,
            teams,
        })
    }

    fn solve(self) -> Result<(u64, u64)> {
        Ok((self.minimum(), self.maximum()))
    }

    fn minimum(&self) -> u64 {
        let members = self.participants / self.teams;
        let one_more_member = self.participants % self.teams;
        let pairs = Self::number_of_pairs(members);
        (self.teams - one_more_member) * pairs + one_more_member * (pairs + members)
    }

    fn maximum(&self) -> u64 {
        Self::number_of_pairs(self.participants - (self.teams - 1))
    }

    fn number_of_pairs(members: u64) -> u64 {
        let n = members - 1;
        n * (n + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Teams {
            participants: 5,
            teams: 1,
        };
        let actual = problem.solve()?;
        assert_eq!((10, 10), actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Teams {
            participants: 3,
            teams: 2,
        };
        let actual = problem.solve()?;
        assert_eq!((1, 1), actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = Teams {
            participants: 6,
            teams: 3,
        };
        let actual = problem.solve()?;
        assert_eq!((3, 6), actual);
        Ok(())
    }
}
