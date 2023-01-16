use std::collections::{HashMap, HashSet};
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
    let problem = Pokemons::from_lines(lines)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Pokemons {
    pokemons: String,
}

impl Pokemons {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next!(lines => let flat_count: usize = 1..=100_000);
        let pokemons = lines.next().ok_or("Missing pokemons line")??;
        Ok(Pokemons { pokemons })
    }

    fn solve(self) -> Result<usize> {
        let pokemons = self.pokemons.as_bytes();
        let type_count = pokemons.iter().collect::<HashSet<_>>().len();

        let mut visited_types: HashMap<_, u32> = HashMap::new();
        let mut visited_minimum = pokemons.len();
        let mut visited = 0;

        let mut left_iterator = pokemons.iter();
        let mut right_iterator = pokemons.iter();
        loop {
            while visited_types.len() < type_count {
                let Some(right) = right_iterator.next() else { break; };
                *visited_types.entry(right).or_default() += 1;
                visited += 1;
            }
            while visited_types.len() == type_count {
                let Some(left) = left_iterator.next() else { break; };
                let entry = visited_types
                    .get_mut(left)
                    .expect("Inserted by right iterator");
                if *entry > 1 {
                    *entry -= 1;
                } else {
                    visited_types.remove(left);
                    visited_minimum = visited_minimum.min(visited);
                }
                visited -= 1;
            }
            if right_iterator.len() == 0 {
                break;
            }
        }

        Ok(visited_minimum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Pokemons {
            pokemons: "AaA".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(2, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Pokemons {
            pokemons: "bcAAcbc".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(3, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = Pokemons {
            pokemons: "aaBCCe".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(5, actual);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let problem = Pokemons {
            pokemons: "AAccaaaAAccAcaaa".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(3, actual);
        Ok(())
    }
}
