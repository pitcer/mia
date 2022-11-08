use std::io;
use std::io::{BufRead, Lines};

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

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let mut lines = stdin.lines();
    parse_next!(lines => let test_cases: u32 = 1..=2 * 10u32.pow(5));

    for _ in 0..test_cases {
        let problem = DeliveryDilemma::from_lines(&mut lines)?;
        let result = problem.solve()?;
        println!("{result}");
    }

    debug_assert!(lines.next().is_none());
    Ok(())
}

struct DeliveryDilemma {
    delivery_times: Vec<u32>,
    pick_up_times: Vec<u32>,
}

impl DeliveryDilemma {
    fn from_lines(lines: &mut Lines<impl BufRead>) -> Result<Self> {
        parse_next!(lines => let dish_count: u32 = 1..=2 * 10u32.pow(5));
        parse_next!(lines => let delivery_times: Vec<_> = [1..=10u32.pow(9); dish_count]);
        parse_next!(lines => let pick_up_times: Vec<_> = [1..=10u32.pow(9); dish_count]);

        Ok(DeliveryDilemma {
            delivery_times,
            pick_up_times,
        })
    }

    fn solve(self) -> Result<u64> {
        let pick_up_times_sum = self
            .pick_up_times
            .iter()
            .map(|pick_up_time| *pick_up_time as u64)
            .sum::<u64>();

        let mut times = self
            .delivery_times
            .into_iter()
            .zip(self.pick_up_times)
            .collect::<Vec<_>>();
        times.sort_unstable_by_key(|(delivery_time, _)| *delivery_time);

        let minimum_time = times
            .into_iter()
            .scan(
                pick_up_times_sum,
                |times_sum, (delivery_time, pick_up_time)| {
                    *times_sum -= pick_up_time as u64;
                    Some(u64::max(delivery_time as u64, *times_sum))
                },
            )
            .min()
            .ok_or("Iterator cannot be empty")?;
        let minimum_time = u64::min(pick_up_times_sum, minimum_time);
        Ok(minimum_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = DeliveryDilemma {
            delivery_times: vec![3, 7, 4, 5],
            pick_up_times: vec![2, 1, 2, 4],
        };
        let actual = problem.solve()?;
        assert_eq!(5, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = DeliveryDilemma {
            delivery_times: vec![1, 2, 3, 4],
            pick_up_times: vec![3, 3, 3, 3],
        };
        let actual = problem.solve()?;
        assert_eq!(3, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = DeliveryDilemma {
            delivery_times: vec![1, 2],
            pick_up_times: vec![10, 10],
        };
        let actual = problem.solve()?;
        assert_eq!(2, actual);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let problem = DeliveryDilemma {
            delivery_times: vec![10, 10],
            pick_up_times: vec![1, 2],
        };
        let actual = problem.solve()?;
        assert_eq!(3, actual);
        Ok(())
    }
}
