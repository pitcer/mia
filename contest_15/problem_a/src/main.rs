use std::io;

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
    parse_next!(lines => let _test_cases: u8 = 1..=100);
    for line in lines {
        let line = line?;
        parse_next!(line -> let cats: usize = 2..=100);
        let result = solve(cats)
            .into_iter()
            .map(|number| number.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{result}");
    }
    Ok(())
}

fn solve(cats: usize) -> Vec<usize> {
    let mut result = vec![0; cats];
    let mut start_index = 0;

    if cats % 2 == 1 {
        result[0..=2].copy_from_slice(&[2, 3, 1]);
        start_index = 3;
    }

    for index in 0..(cats - start_index) / 2 {
        let index = 2 * index + start_index;
        result[index..=(index + 1)].copy_from_slice(&[index + 2, index + 1]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let actual = solve(2);
        assert_eq!(vec![2, 1], actual);
    }

    #[test]
    fn test_example_2() {
        let actual = solve(3);
        assert_eq!(vec![2, 3, 1], actual);
    }

    #[test]
    fn test_example_3() {
        let actual = solve(5);
        assert_eq!(vec![2, 3, 1, 5, 4], actual);
    }
}
