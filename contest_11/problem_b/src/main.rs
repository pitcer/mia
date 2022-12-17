use std::io;

macro_rules! parse_next {
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
        let problem = NumberGame::from_line(line?)?;
        let result = problem.solve()?;
        if result {
            println!("Ashishgup");
        } else {
            println!("FastestFinger");
        }
    }
    Ok(())
}

struct NumberGame(u32);

impl NumberGame {
    fn from_line(line: String) -> Result<Self> {
        parse_next!(line -> let n: u32 = 1..=10u32.pow(9));
        Ok(NumberGame(n))
    }

    /// Returns true if the first player wins.
    fn solve(self) -> Result<bool> {
        let (twos, odd_part) = self.extract_twos();
        debug_assert!(odd_part > 0);
        if odd_part == 1 {
            Ok(twos == 1)
        } else if twos == 1 {
            Ok(!Self::is_prime(odd_part))
        } else {
            Ok(true)
        }
    }

    /// Returns number of twos and the remaining odd part.
    fn extract_twos(&self) -> (u8, u32) {
        let mut twos = 0;
        let mut remaining = self.0;
        while remaining % 2 == 0 {
            twos += 1;
            remaining /= 2;
        }
        (twos, remaining)
    }

    fn is_prime(n: u32) -> bool {
        let mut divisor = 2;
        while divisor * divisor <= n {
            if n % divisor == 0 {
                return false;
            }
            divisor += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() -> Result<()> {
        assert_eq!(false, NumberGame(1).solve()?);
        assert_eq!(true, NumberGame(2).solve()?);
        assert_eq!(true, NumberGame(3).solve()?);
        assert_eq!(false, NumberGame(4).solve()?);
        assert_eq!(true, NumberGame(5).solve()?);
        assert_eq!(false, NumberGame(6).solve()?);
        assert_eq!(true, NumberGame(12).solve()?);
        Ok(())
    }
}
