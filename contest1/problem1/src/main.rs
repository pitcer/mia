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

    // >=1.60.0: usize::abs_diff(even_integers_count, odd_integers_count)
    if isize::abs(even_integers_count as isize - odd_integers_count as isize) <= 1 {
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
