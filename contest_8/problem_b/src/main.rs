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
    let problem = Benches::from_lines(lines)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Benches {
    paths: u64,
}

impl Benches {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next!(lines => let paths: u64 = 5..=100);
        Ok(Benches { paths })
    }

    /// The result is (p choose b)^2 * b!, where p is the number of paths and b is the number of
    /// benches.
    /// At first, we choose in what columns and rows we'll be putting benches using binomial
    /// coefficient. Then on those 2-dimensional bxb sub-arrays we count all possible bench
    /// arrangements using factorial.
    fn solve(self) -> Result<u64> {
        const BENCHES: u64 = 5;
        const BENCHES_FACTORIAL: u64 = 2 * 3 * 4 * 5;
        let result = ((self.paths - BENCHES + 1)..=self.paths)
            .into_iter()
            .product::<u64>();
        let result = (result / BENCHES_FACTORIAL) * result;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let problem = Benches { paths: 5 };
        let actual = problem.solve()?;
        assert_eq!(120, actual);
        Ok(())
    }
}
