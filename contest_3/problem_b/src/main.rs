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
    let problem = Palindromes::from_lines(lines)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Palindromes {
    palindromes_count: u32,
    modulus: u64,
}

impl Palindromes {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let palindromes_count: u32 = 1..=10u32.pow(5);
            let modulus: u64 = 1..=10u64.pow(9)
        }

        Ok(Palindromes {
            palindromes_count,
            modulus,
        })
    }

    fn solve(self) -> Result<u32> {
        let mut sum = 0;
        for number in 1..=self.palindromes_count {
            let number_string = number.to_string();
            let number_reversed = number_string.chars().rev().collect::<String>();
            let palindrome = format!("{number_string}{number_reversed}");
            let palindrome = palindrome.parse::<u64>()? % self.modulus;
            sum = (sum + palindrome) % self.modulus;
        }
        Ok(sum as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Palindromes {
            palindromes_count: 2,
            modulus: 100,
        };
        let actual = problem.solve()?;
        assert_eq!(33, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Palindromes {
            palindromes_count: 5,
            modulus: 30,
        };
        let actual = problem.solve()?;
        assert_eq!(15, actual);
        Ok(())
    }
}
