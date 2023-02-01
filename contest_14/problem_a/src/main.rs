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
    parse_next!(lines => let vertex_count: usize = 1..=4);
    let vertices = (0..vertex_count)
        .map(|_| {
            parse_next!(lines => let x: i16 = -1000..=1000; let y: i16 = -1000..=1000);
            Ok((x, y))
        })
        .collect::<Result<Vec<_>>>()?;
    let result = solve(vertices).map(|result| result as i32).unwrap_or(-1);
    println!("{result}");
    Ok(())
}

fn solve(vertices: Vec<(i16, i16)>) -> Option<u32> {
    if vertices.len() < 2 {
        return None;
    }

    let (x_1, y_1) = vertices[0];
    let (x_2, y_2) = vertices[1];

    if x_1 != x_2 && y_1 != y_2 {
        return Some(i16::abs(x_2 - x_1) as u32 * i16::abs(y_2 - y_1) as u32);
    }

    if vertices.len() < 3 {
        return None;
    }

    let (x_3, y_3) = vertices[2];

    if x_3 != x_2 && y_3 != y_2 {
        return Some(i16::abs(x_2 - x_3) as u32 * i16::abs(y_2 - y_3) as u32);
    }

    if x_3 != x_1 && y_3 != y_1 {
        return Some(i16::abs(x_1 - x_3) as u32 * i16::abs(y_1 - y_3) as u32);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, vec};

    #[test]
    fn test_example_1() {
        let actual = solve(vec![(0, 0), (1, 1)]);
        assert_eq!(Some(1), actual);
    }

    #[test]
    fn test_example_2() {
        let actual = solve(vec![(1, 1)]);
        assert_eq!(None, actual);
    }
}
