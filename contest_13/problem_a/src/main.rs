use std::io;
use std::ops::BitOr;

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
    let mut lines = stdin.lines();
    parse_next!(lines => let _cases: u16 = 1..=1000);
    while let Some(line) = lines.next() {
        let line = line?;
        parse_next!(line -> let number_count: usize = 2..=100);
        parse_next!(lines => let numbers: Vec<u32> = [0..=2u32.pow(30); number_count]);
        let result = solve(numbers);
        println!("{result}");
    }
    Ok(())
}

fn solve(numbers: Vec<u32>) -> u32 {
    numbers
        .into_iter()
        .reduce(u32::bitor)
        .expect("len(numbers) >= 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let actual = solve(vec![1, 3, 2]);
        assert_eq!(3, actual);
    }

    #[test]
    fn test_example_2() {
        let actual = solve(vec![1, 2, 4, 8, 16]);
        assert_eq!(31, actual);
    }

    #[test]
    fn test_example_3() {
        let actual = solve(vec![6, 6]);
        assert_eq!(6, actual);
    }

    #[test]
    fn test_example_4() {
        let actual = solve(vec![3, 5, 6]);
        assert_eq!(7, actual);
    }
}
