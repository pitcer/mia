use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let problem = Orchestra::from_reader(stdin)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Orchestra {
    rows: u8,
    columns: u8,
    minimum_viola_count: u8,
    violas: HashSet<(u8, u8)>,
}

impl Problem<u32> for Orchestra {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        fn split_parse_iter<T>(line: &str) -> impl Iterator<Item = Result<T, T::Err>> + '_
        where
            T: FromStr,
            T::Err: std::error::Error,
        {
            line.split_whitespace().map(|entry| entry.parse::<T>())
        }

        let mut lines = reader.lines();

        let first_line = lines.next().ok_or("Missing first line")??;
        let mut first_line = split_parse_iter::<u8>(&first_line);
        let rows = first_line.next().ok_or("Missing rows")??;
        let columns = first_line.next().ok_or("Missing columns")??;
        let viola_count = first_line.next().ok_or("Missing viola count")??;
        let minimum_viola_count = first_line.next().ok_or("Missing minimum viola count")??;

        debug_assert!((1..=10).contains(&rows));
        debug_assert!((1..=10).contains(&columns));
        debug_assert!((1..=10).contains(&viola_count));
        debug_assert!((1..=viola_count).contains(&minimum_viola_count));
        debug_assert!(first_line.next().is_none());

        let mut violas = HashSet::with_capacity(viola_count as usize);
        for line in lines {
            let line = line?;
            let mut line = split_parse_iter::<u8>(&line);
            let x = line.next().ok_or("Missing x")??;
            let y = line.next().ok_or("Missing y")??;
            violas.insert((x - 1, y - 1));
        }

        debug_assert_eq!(violas.len(), viola_count as usize);
        for (x, y) in &violas {
            debug_assert!((0..=rows - 1).contains(x));
            debug_assert!((0..=columns - 1).contains(y));
        }

        Ok(Orchestra {
            rows,
            columns,
            minimum_viola_count,
            violas,
        })
    }

    fn solve(self) -> Result<u32> {
        let mut photo_count = 0;

        for row in 0..self.rows {
            for column in 0..self.columns {
                for width in 1..=self.columns - column {
                    for height in 1..=self.rows - row {
                        let viola_count =
                            self.count_violas_in_rectangle(column, row, width, height);

                        if viola_count >= self.minimum_viola_count {
                            photo_count += 1;
                        }
                    }
                }
            }
        }

        Ok(photo_count)
    }
}

impl Orchestra {
    fn count_violas_in_rectangle(&self, column: u8, row: u8, width: u8, height: u8) -> u8 {
        let mut viola_count = 0;
        for x in column..column + width {
            for y in row..row + height {
                if self.violas.contains(&(y, x)) {
                    viola_count += 1;
                }
            }
        }
        viola_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Orchestra {
            rows: 2,
            columns: 2,
            minimum_viola_count: 1,
            violas: HashSet::from([(0, 1)]),
        };
        let actual = problem.solve()?;
        assert_eq!(4, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Orchestra {
            rows: 3,
            columns: 2,
            minimum_viola_count: 3,
            violas: HashSet::from([(0, 0), (2, 0), (1, 1)]),
        };
        let actual = problem.solve()?;
        assert_eq!(1, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = Orchestra {
            rows: 3,
            columns: 2,
            minimum_viola_count: 2,
            violas: HashSet::from([(0, 0), (2, 0), (1, 1)]),
        };
        let actual = problem.solve()?;
        assert_eq!(4, actual);
        Ok(())
    }
}
