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
    let lines = stdin.lines();
    let problem = Fax::from_lines(lines)?;
    let result = problem.solve()?;
    if result {
        println!("YES");
    } else {
        println!("NO");
    }
    Ok(())
}

struct Fax {
    string: String,
    palindromes: usize,
}

impl Fax {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        let string = lines.next().ok_or("Missing line")??;
        parse_next!(lines => let palindromes: usize = 1..=1000);
        Ok(Fax {
            string,
            palindromes,
        })
    }

    fn solve(self) -> Result<bool> {
        if self.string.len() % self.palindromes != 0 {
            return Ok(false);
        }
        let palindrome_size = self.string.len() / self.palindromes;
        Ok(self
            .string
            .as_bytes()
            .chunks_exact(palindrome_size)
            .all(Self::is_palindrome))
    }

    fn is_palindrome(string: &[u8]) -> bool {
        for index in 0..(string.len() / 2) {
            if string[index] != string[string.len() - index - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Fax {
            string: "saba".to_owned(),
            palindromes: 2,
        };
        let actual = problem.solve()?;
        assert_eq!(false, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Fax {
            string: "saddastavvat".to_owned(),
            palindromes: 2,
        };
        let actual = problem.solve()?;
        assert_eq!(true, actual);
        Ok(())
    }
}
