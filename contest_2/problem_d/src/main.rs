use std::io::BufRead;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // counterexample
    // 6 3
    // 1 4
    // 4 6
    // 1 5
    // 5 2
    // 3 6
    // out: 1 2 5
    #[test]
    fn test_counterexample() -> Result<()> {
        Ok(())
    }

    // TLE on n^2 algorithms
    // 1000000 500000
    // 1000000 1
    // 1 2
    // 2 3
    // ...
    // 499998 499999
    // 499999 500000
    // 499999 500001
    // 499999 500002
    // ...
    // 499999 999999
    // out: 500000 ... 999999
    #[test]
    fn test_broomstick() -> Result<()> {
        Ok(())
    }
}
