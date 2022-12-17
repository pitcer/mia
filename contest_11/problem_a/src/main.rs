extern crate core;

use std::io;
use std::io::{BufRead, Lines};

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lines();
    let problem = Game::from_lines(lines)?;
    let result = problem.solve()?;
    if result {
        println!("First");
    } else {
        println!("Second");
    }
    Ok(())
}

struct Game {
    string: String,
}

impl Game {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        let string = lines.next().ok_or("Missing line")??;
        Ok(Game { string })
    }

    fn solve(self) -> Result<bool> {
        let letters = self.count_letters();
        let odds = letters.iter().filter(|count| *count % 2 == 1).count();
        Ok(odds.saturating_sub(1) % 2 == 0)
    }

    fn count_letters(&self) -> [u16; 26] {
        let mut letters = [0; 26];
        for character in self.string.chars() {
            let index = character as usize - 'a' as usize;
            letters[index] += 1;
        }
        letters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Game {
            string: "aba".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(true, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Game {
            string: "abca".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(false, actual);
        Ok(())
    }

    #[test]
    fn test_no_odds() -> Result<()> {
        let problem = Game {
            string: "aa".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(true, actual);
        Ok(())
    }
}
