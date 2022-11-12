use std::collections::HashMap;
use std::io::{BufRead, Lines};
use std::{cmp, io};

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
    let message = Message::from_lines(lines)?;
    let result = message.solve()?;
    println!("{result}");
    Ok(())
}

struct Message {
    group_count: u32,
    words: String,
    words_costs: Vec<u32>,
    words_groups: Vec<usize>,
    message: String,
}

impl Message {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let word_count: u32 = 1..=10u32.pow(5);
            let group_count: u32 = 1..=word_count;
            let message_word_count: u32 = 1..=10u32.pow(5)
        }
        let words = lines.next().ok_or("Missing words line")??;
        parse_next!(lines => let words_costs: Vec<_> = [1..=10u32.pow(9); word_count]);

        let mut words_groups = vec![None; word_count as usize];
        for group_index in 0..group_count as usize {
            let line = lines.next().ok_or("Missing next line")??;
            let mut line = line.split_whitespace().map(|entry| entry.parse::<u32>());
            let _group_length = line.next().ok_or("Missing group length")??;

            for group_member in line {
                let group_id = group_member?;
                debug_assert!((1..=word_count).contains(&group_id));
                words_groups[group_id as usize - 1] = Some(group_index + 1);
            }
        }
        let words_groups = words_groups
            .into_iter()
            .collect::<Option<Vec<usize>>>()
            .ok_or("Missing group id for some word")?;

        let message = lines.next().ok_or("Missing message line")??;

        Ok(Message {
            group_count,
            words,
            words_costs,
            words_groups,
            message,
        })
    }

    fn solve(self) -> Result<u64> {
        let mut groups_costs: Vec<Option<u32>> = vec![None; self.group_count as usize];
        for (word_index, group_id) in self.words_groups.iter().enumerate() {
            let word_cost = self.words_costs[word_index];
            let group_cost = &mut groups_costs[group_id - 1];
            let cost = group_cost.map_or(word_cost, |cost| cmp::min(cost, word_cost));
            *group_cost = Some(cost);
        }
        let groups_costs = groups_costs
            .into_iter()
            .collect::<Option<Vec<u32>>>()
            .ok_or("Missing cost for some group")?;

        let words_to_id = self
            .words
            .split_whitespace()
            .enumerate()
            .map(|(index, word)| (word, index + 1))
            .collect::<HashMap<_, _>>();
        let total_cost = self
            .message
            .split_whitespace()
            .map(|word| words_to_id[word])
            .map(|word_id| self.words_groups[word_id - 1])
            .map(|group_id| groups_costs[group_id - 1] as u64)
            .sum();
        Ok(total_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Message {
            group_count: 4,
            words: "i loser am the second".to_owned(),
            words_costs: vec![100, 1, 1, 5, 10],
            words_groups: vec![1, 3, 2, 4, 3],
            message: "i am the second".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(107, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Message {
            group_count: 4,
            words: "i loser am the second".to_owned(),
            words_costs: vec![100, 20, 1, 5, 10],
            words_groups: vec![1, 3, 2, 4, 3],
            message: "i am the second".to_owned(),
        };
        let actual = problem.solve()?;
        assert_eq!(116, actual);
        Ok(())
    }
}
