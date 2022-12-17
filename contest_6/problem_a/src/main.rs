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
    let message = LectureSleep::from_lines(lines)?;
    let result = message.solve()?;
    println!("{result}");
    Ok(())
}

struct LectureSleep {
    awake_duration: usize,
    theorems: Vec<u32>,
    behaviour: Vec<u8>,
}

impl LectureSleep {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let lecture_duration: usize = 1..=10usize.pow(5);
            let awake_duration: usize = 1..=lecture_duration
        }
        parse_next!(lines => let theorems: Vec<_> = [1..=10u32.pow(4); lecture_duration]);
        parse_next!(lines => let behaviour: Vec<u8> = [0..=1; lecture_duration]);

        Ok(LectureSleep {
            awake_duration,
            theorems,
            behaviour,
        })
    }

    fn solve(self) -> Result<u32> {
        let technique_start_index = self
            .theorems
            .windows(self.awake_duration)
            .enumerate()
            .map(|(index, theorems)| {
                let theorems_by_technique = theorems
                    .iter()
                    .enumerate()
                    .filter(|(theorem_index, _)| self.behaviour[index + theorem_index] == 0)
                    .map(|(_, theorems)| theorems)
                    .sum::<u32>();
                (index, theorems_by_technique)
            })
            .max_by_key(|(_, theorems_by_technique)| *theorems_by_technique)
            .map(|(index, _)| index)
            .ok_or("Missing element in theorems list")?;
        let theorems_learned = self
            .theorems
            .into_iter()
            .enumerate()
            .filter(|(index, _)| {
                self.behaviour[*index] == 1
                    || (technique_start_index..technique_start_index + self.awake_duration)
                        .contains(index)
            })
            .map(|(_, theorems)| theorems)
            .sum();
        Ok(theorems_learned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let problem = LectureSleep {
            awake_duration: 3,
            theorems: vec![1, 3, 5, 2, 5, 4],
            behaviour: vec![1, 1, 0, 1, 0, 0],
        };
        let actual = problem.solve()?;
        assert_eq!(16, actual);
        Ok(())
    }
}
