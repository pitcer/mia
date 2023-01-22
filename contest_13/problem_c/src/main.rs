use std::{cmp, io, mem};

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
    parse_next!(lines => let _tests: u16 = 1..=1000);
    while let Some(line) = lines.next() {
        let line = line?;
        parse_next!(line -> let bag: u64 = 1..=10u64.pow(18));
        parse_next!(line -> let box_count: usize = 1..=10usize.pow(5));
        parse_next!(lines => let boxes: Vec<u32> = [1..=10u32.pow(9); box_count]);
        let result = solve(bag, boxes).map(|result| result as i32).unwrap_or(-1);
        println!("{result}");
    }
    Ok(())
}

fn solve(bag: u64, boxes: Vec<u32>) -> Option<u32> {
    let bag_length = mem::size_of_val(&bag) * 8 - bag.leading_zeros() as usize;
    let boxes_length = cmp::max(mem::size_of::<u32>() * 8, bag_length);

    let mut boxes = boxes
        .into_iter()
        .fold(vec![0; boxes_length + 1], |mut boxes, current| {
            let index = current.trailing_zeros() as usize;
            boxes[index] += 1;
            boxes
        });

    let mut all_divisions = 0;
    let mut divisions = 0;

    let mut shift = 0;
    while shift < boxes_length && (shift < bag_length || divisions > 0) {
        let mut increment_divisions = false;

        if divisions > 0 {
            if boxes[shift] > 0 {
                all_divisions += divisions;
                divisions = 0;
                boxes[shift] -= 1;
            } else {
                increment_divisions = true;
            }
        }

        if (bag >> shift) & 0b1 == 1 {
            if boxes[shift] > 0 {
                boxes[shift] -= 1;
            } else {
                increment_divisions = true;
            }
        }

        boxes[shift + 1] += boxes[shift] / 2;
        divisions += u32::from(increment_divisions);
        shift += 1;
    }

    (divisions == 0).then_some(all_divisions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let actual = solve(10, vec![1, 32, 1]);
        assert_eq!(Some(2), actual);
    }

    #[test]
    fn test_example_2() {
        let actual = solve(23, vec![16, 1, 4, 1]);
        assert_eq!(None, actual);
    }

    #[test]
    fn test_example_3() {
        let actual = solve(20, vec![2, 1, 16, 1, 8]);
        assert_eq!(Some(0), actual);
    }
}
