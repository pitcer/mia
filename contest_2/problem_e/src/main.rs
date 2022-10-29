use std::io;
use std::io::BufRead;
use std::str::FromStr;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let problem = ColoredBalls::from_reader(stdin)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct ColoredBalls {
    boxes: Vec<u32>,
}

impl Problem<u32> for ColoredBalls {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        fn split_parse_iter<T>(line: &str) -> impl Iterator<Item = Result<T, T::Err>> + '_
        where
            T: FromStr,
            T::Err: std::error::Error,
        {
            line.split_whitespace().map(|entry| entry.parse::<T>())
        }

        let mut lines = reader.lines();

        let first_line = lines.next().ok_or("Missing first line")??;
        let mut first_line = split_parse_iter(&first_line);
        let box_count = first_line.next().ok_or("Missing box count")??;

        debug_assert!((1..=500).contains(&box_count));
        debug_assert!(first_line.next().is_none());

        let second_line = lines.next().ok_or("Missing second line")??;
        let second_line = split_parse_iter(&second_line);
        let boxes = second_line.collect::<Result<Vec<_>, _>>()?;

        debug_assert_eq!(boxes.len(), box_count);
        for balls_count in &boxes {
            debug_assert!((1..=10u32.pow(9)).contains(balls_count))
        }

        Ok(ColoredBalls { boxes })
    }

    fn solve(self) -> Result<u32> {
        fn div_ceil(a: u32, b: u32) -> u32 {
            (a + b - 1) / b
        }

        let minimum = self
            .boxes
            .iter()
            .min()
            .expect("Set cannot be empty because of preconditions");
        let maximum_set_size = minimum + 1;

        let set_count = self
            .boxes
            .into_iter()
            .map(|balls_count| div_ceil(balls_count, maximum_set_size))
            .sum::<u32>();

        Ok(set_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = ColoredBalls {
            boxes: vec![4, 7, 8],
        };
        let actual = problem.solve()?;
        assert_eq!(5, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = ColoredBalls { boxes: vec![2, 7] };
        let actual = problem.solve()?;
        assert_eq!(4, actual);
        Ok(())
    }
}
