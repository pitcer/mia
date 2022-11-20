use std::io;
use std::io::{BufRead, Lines};

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
}

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lines();
    let problem = AlternatingSum::from_lines(lines)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

const MODULUS: i64 = 10i64.pow(9) + 9;

struct AlternatingSum {
    n: i64,
    a: i64,
    b: i64,
    k: i64,
    is_positive: Vec<bool>,
}

impl AlternatingSum {
    fn from_lines(mut lines: Lines<impl BufRead>) -> Result<Self> {
        parse_next! { lines =>
            let n: i64 = 1..=10i64.pow(9);
            let a: i64 = 1..=10i64.pow(9);
            let b: i64 = 1..=10i64.pow(9);
            let k: i64 = 1..=10i64.pow(5)
        }
        let sequence = lines.next().ok_or("Sequence line missing")??;
        let is_positive = sequence
            .chars()
            .map(|sign| {
                if sign == '+' {
                    true
                } else if sign == '-' {
                    false
                } else {
                    panic!("Invalid sign character")
                }
            })
            .collect();

        Ok(AlternatingSum {
            n,
            a,
            b,
            k,
            is_positive,
        })
    }

    /// Let p = MODULUS. Let s_i = 1 for is_positive == true or -1 for is_positive == false.
    /// We have to calculate Σ_{i=0}^{n} s_i * a^(n-i) * b^i (mod p).
    fn solve(self) -> Result<i64> {
        let a_inverse = modular_inverse(self.a);
        let a_b = (a_inverse * self.b).rem_euclid(MODULUS);
        let internal_sum = self
            .is_positive
            .into_iter()
            .enumerate()
            .map(|(index, is_positive)| {
                let term = pow_mod(a_b, index as i64);
                if is_positive {
                    term
                } else {
                    (-term).rem_euclid(MODULUS)
                }
            })
            .fold(0i64, |accumulator, item| {
                (accumulator + item).rem_euclid(MODULUS)
            });

        let terms = (self.n + 1) / self.k;
        let ratio = pow_mod(a_b, self.k);
        let a_to_n = pow_mod(self.a, self.n);
        let external_sum = if ratio == 1 {
            (a_to_n * terms).rem_euclid(MODULUS)
        } else {
            (a_to_n * geometric_sum(terms, ratio)).rem_euclid(MODULUS)
        };

        Ok((internal_sum * external_sum).rem_euclid(MODULUS))
    }
}

fn geometric_sum(terms: i64, ratio: i64) -> i64 {
    let inverse = (ratio - 1).rem_euclid(MODULUS);
    let ratio_power = pow_mod(ratio, terms);
    let numerator = (ratio_power - 1).rem_euclid(MODULUS);
    div_mod(numerator, inverse)
}

/// Let p = MODULUS. Divides two numbers mod p.
/// Since p is prime, we can compute modular inverse instead of using division.
fn div_mod(first: i64, second: i64) -> i64 {
    let inverse = modular_inverse(second);
    (first * inverse).rem_euclid(MODULUS)
}

/// Let n = number and p = MODULUS. This function calculates n^(-1) (mod p).
///
/// We know from Fermat's little theorem that n^(p-1) === 1 (mod p) where p is prime.
/// Multiplying both sides by n^(-1) we have n^(-1) === n^(p-2) (mod p).
/// So we have to calculate a^(p-2) (mod p).
fn modular_inverse(number: i64) -> i64 {
    pow_mod(number, MODULUS - 2)
}

fn pow_mod(mut base: i64, mut exponent: i64) -> i64 {
    let mut result = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base).rem_euclid(MODULUS);
        }
        base = (base * base).rem_euclid(MODULUS);
        exponent /= 2;
    }

    result.rem_euclid(MODULUS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = AlternatingSum {
            n: 2,
            a: 2,
            b: 3,
            k: 3,
            is_positive: vec![true, false, true],
        };
        let actual = problem.solve()?;
        assert_eq!(7, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = AlternatingSum {
            n: 4,
            a: 1,
            b: 5,
            k: 1,
            is_positive: vec![false],
        };
        let actual = problem.solve()?;
        assert_eq!(999999228, actual);
        Ok(())
    }

    #[test]
    fn test_pow_mod() -> Result<()> {
        assert_eq!(243, pow_mod(3, 5));
        assert_eq!(738040741, pow_mod(5, 97));
        Ok(())
    }

    #[test]
    fn test_modular_inverse() -> Result<()> {
        assert_eq!(149163232, modular_inverse(4123));
        Ok(())
    }
}
