use std::io;
use std::io::BufRead;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let problem = Trip::from_reader(stdin)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Trip {
    city_count: u32,
    tank_capacity: u32,
}

impl Problem<u32> for Trip {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        fn split_line(line: &str) -> Result<Vec<u32>> {
            let result = line
                .split_whitespace()
                .map(|integer| integer.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(result)
        }

        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        let first_line = lines.get(0).ok_or("Missing first line")?;
        let first_line = split_line(first_line)?;

        let city_count = *first_line.first().ok_or("Missing city count")?;
        let tank_capacity = *first_line.get(1).ok_or("Missing tank capacity")?;

        debug_assert!((2..=100).contains(&city_count));
        debug_assert!((1..=100).contains(&tank_capacity));

        Ok(Trip {
            city_count,
            tank_capacity,
        })
    }

    /// Returns minimum fuel cost.
    ///
    /// Strategy is to keep the tank at its maximum level in every city,
    /// until we reach the city such that at the last city we will have no fuel.
    ///
    /// Let `n` be the number of cities.
    /// Let `v` be the fuel tank capacity.
    /// Let `c(i)` be the cost function such that:
    /// * `c(1) = v`, since we refuel tank to the maximum level at the first city;
    /// * `c(i) = i`, for `i ∈ {2, ..., n - v}`, since we refuel 1 liter at every city;
    /// * `c(i) = 0`, for `i ∈ {n - v + 1, ..., n}`, since the last v cities will deplete our tank.
    ///
    /// Then total fuel cost is:
    /// `∑_{i=1}^{n} c(i) = v + (∑_{i=2}^{n-v} i) + 0 = v + (n - v)(n - v + 1)/2 - 1`
    fn solve(self) -> Result<u32> {
        let n = self.city_count;
        let v = self.tank_capacity;

        if n < v {
            Ok(n - 1)
        } else {
            let n_minus_v = n - v;
            Ok(v + (n_minus_v * (n_minus_v + 1)) / 2 - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Trip {
            city_count: 4,
            tank_capacity: 2,
        };
        let actual = problem.solve()?;
        assert_eq!(4, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Trip {
            city_count: 7,
            tank_capacity: 6,
        };
        let actual = problem.solve()?;
        assert_eq!(6, actual);
        Ok(())
    }
}
