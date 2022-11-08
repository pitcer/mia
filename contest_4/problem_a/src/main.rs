use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use std::vec::IntoIter;

macro_rules! parse_next {
    ($lines:ident => let $name:ident: $ty:ty = [$range:expr; $count:expr]) => {
        let line = $lines.next().ok_or("Missing next line")??;
        let line = line.split_whitespace().map(|entry| entry.parse::<_>());
        let $name = line.collect::<Result<$ty, _>>()?;

        debug_assert_eq!($name.len(), $count as usize);
        for item in &$name {
            debug_assert!(($range).contains(item));
        }
    };
    ($lines:ident => $(let $name:ident: $ty:ty = $range:expr);+) => {
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

trait Problem
where
    Self: Sized,
{
    type Output;

    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<Self::Output>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let problem = Siege::from_reader(stdin)?;
    let result = problem.solve()?;
    for standing_warriors in result {
        println!("{standing_warriors}")
    }
    Ok(())
}

struct Siege {
    warriors_strengths: Vec<u32>,
    arrows: Vec<u64>,
}

impl Problem for Siege {
    type Output = StandingWarriors;

    fn from_reader(reader: impl BufRead) -> Result<Self> {
        let mut lines = reader.lines();
        parse_next! { lines =>
            let warrior_count: u32 = 1..=200_000;
            let battle_time: u32 = 1..=200_000
        }
        parse_next!(lines => let warriors_strengths: Vec<_> = [1..=10u32.pow(9); warrior_count]);
        parse_next!(lines => let arrows: Vec<_> = [1..=10u64.pow(14); battle_time]);
        debug_assert!(lines.next().is_none());

        Ok(Siege {
            warriors_strengths,
            arrows,
        })
    }

    fn solve(self) -> Result<StandingWarriors> {
        let strength_prefix_sums = self
            .warriors_strengths
            .into_iter()
            .scan(0u64, |accumulator, value| {
                *accumulator += value as u64;
                Some(*accumulator)
            })
            .collect();
        let warriors = StandingWarriors {
            strength_prefix_sums,
            arrows_iterator: self.arrows.into_iter(),
            arrows_sum: 0,
            killed_warriors: 0,
        };
        Ok(warriors)
    }
}

struct StandingWarriors {
    strength_prefix_sums: Vec<u64>,
    arrows_iterator: IntoIter<u64>,
    arrows_sum: u64,
    killed_warriors: usize,
}

impl Iterator for StandingWarriors {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let arrows = self.arrows_iterator.next()?;
        self.arrows_sum += arrows;

        let previously_killed_warriors = self.killed_warriors;
        let subslice = &self.strength_prefix_sums[previously_killed_warriors..];
        let killed_warriors = match subslice.binary_search(&self.arrows_sum) {
            Ok(killed_warriors) => killed_warriors + 1,
            Err(killed_warriors) => killed_warriors,
        };
        let killed_warriors = previously_killed_warriors + killed_warriors;

        let warrior_count = self.warrior_count();
        let standing_warriors = match warrior_count.cmp(&killed_warriors) {
            Ordering::Less | Ordering::Equal => {
                self.killed_warriors = 0;
                self.arrows_sum = 0;
                warrior_count
            }
            Ordering::Greater => {
                self.killed_warriors = killed_warriors;
                warrior_count - killed_warriors
            }
        };

        Some(standing_warriors as u32)
    }
}

impl StandingWarriors {
    fn warrior_count(&self) -> usize {
        self.strength_prefix_sums.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Siege {
            warriors_strengths: vec![1, 2, 1, 2, 1],
            arrows: vec![3, 10, 1, 1, 1],
        };
        let actual = problem.solve()?.collect::<Vec<u32>>();
        assert_eq!(vec![3, 5, 4, 4, 3], actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Siege {
            warriors_strengths: vec![1, 2, 3, 4],
            arrows: vec![9, 1, 10, 6],
        };
        let actual = problem.solve()?.collect::<Vec<u32>>();
        assert_eq!(vec![1, 4, 4, 1], actual);
        Ok(())
    }
}
