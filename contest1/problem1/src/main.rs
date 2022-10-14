use std::io;
use std::io::BufRead;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

fn main() -> Result<()> {
    let integers = get_input()?;
    let sum = minimum_possible_sum(integers)?;
    println!("{sum}");
    Ok(())
}

fn get_input() -> Result<Vec<u32>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let lines = stdin.lines().collect::<Result<Vec<_>, _>>()?;
    assert_eq!(lines.len(), 2);

    let integers_count_line = lines.get(0).ok_or("Missing first line")?;
    let integers_line = lines.get(1).ok_or("Missing second line")?;

    let integers_count = integers_count_line.parse::<usize>()?;
    let integers = integers_line
        .split_whitespace()
        .map(|integer| integer.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(integers.len(), integers_count);

    Ok(integers)
}

fn minimum_possible_sum(integers: Vec<u32>) -> Result<u32> {
    let integers_count = integers.len();
    let (even_integers, odd_integers) = integers
        .into_iter()
        .partition::<Vec<_>, _>(|integer| *integer % 2 == 0);
    let even_integers_count = even_integers.len();
    let odd_integers_count = odd_integers.len();

    if usize::abs_diff(even_integers_count, odd_integers_count) <= 1 {
        return Ok(0);
    }

    let (mut integers, take_count) = if even_integers_count > odd_integers_count {
        (even_integers, integers_count - (2 * odd_integers_count + 1))
    } else {
        (odd_integers, integers_count - (2 * even_integers_count + 1))
    };

    integers.sort_unstable();
    let sum = integers.into_iter().take(take_count).sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let actual = minimum_possible_sum(vec![1, 5, 7, 8, 2])?;
        assert_eq!(0, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let actual = minimum_possible_sum(vec![5, 1, 2, 4, 6, 3])?;
        assert_eq!(0, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let actual = minimum_possible_sum(vec![1000000, 1000000])?;
        assert_eq!(1000000, actual);
        Ok(())
    }
}
