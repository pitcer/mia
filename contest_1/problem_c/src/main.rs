use std::io::BufRead;
use std::ops::Range;
use std::{cmp, io};

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let problem = TransposingMatrices::from_reader(stdin)?;
    let result = problem.solve()?;
    let result = if result { "YES" } else { "NO" };
    println!("{result}");
    Ok(())
}

#[derive(Debug)]
struct TransposingMatrices {
    row_count: usize,
    column_count: usize,
    before: Matrix,
    after: Matrix,
}

#[derive(Debug)]
struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Problem<bool> for TransposingMatrices {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        fn split_line(line: &str) -> Result<Vec<u32>> {
            let result = line
                .split_whitespace()
                .map(|integer| integer.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(result)
        }

        fn read_rows(lines: &[String], lines_range: Range<usize>) -> Result<Matrix> {
            let rows = lines_range
                .map(|index| {
                    let row = lines
                        .get(index)
                        .ok_or_else(|| format!("Missing {index} line"))?;
                    split_line(row)
                })
                .collect::<Result<Vec<_>, _>>()?;

            for row in &rows {
                for element in row {
                    debug_assert!((1..=10u32.pow(9)).contains(element))
                }
            }

            Ok(Matrix { rows })
        }

        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;

        let first_line = lines.get(0).ok_or("Missing first line")?;
        let first_line = split_line(first_line)?;

        let row_count = *first_line.first().ok_or("Missing row count")? as usize;
        let column_count = *first_line.get(1).ok_or("Missing column count")? as usize;

        debug_assert!((1..=500).contains(&row_count));
        debug_assert!((1..=500).contains(&column_count));

        let before = read_rows(&lines, 1..row_count + 1)?;
        let after = read_rows(&lines, row_count + 1..2 * row_count + 1)?;

        Ok(TransposingMatrices {
            row_count,
            column_count,
            before,
            after,
        })
    }

    /// Returns true if self.after can be achieved by transpositions in self.before.
    fn solve(self) -> Result<bool> {
        for index in 0..self.column_count + self.row_count - 1 {
            let mut before_diagonal = self.before.get_reverse_diagonal(index);
            before_diagonal.sort_unstable();
            let mut after_diagonal = self.after.get_reverse_diagonal(index);
            after_diagonal.sort_unstable();
            if before_diagonal != after_diagonal {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl Matrix {
    fn get_reverse_diagonal(&self, index: usize) -> Vec<u32> {
        debug_assert!(index < self.column_count() + self.row_count() - 1);

        let mut result = Vec::with_capacity(cmp::max(self.column_count(), self.row_count()));
        let (mut row_index, mut column_index) = self.diagonal_index_to_element_index(index);
        while column_index < self.column_count() {
            result.push(self.rows[row_index][column_index]);
            if row_index == 0 {
                break;
            }
            row_index -= 1;
            column_index += 1;
        }
        result
    }

    fn diagonal_index_to_element_index(&self, index: usize) -> (usize, usize) {
        let row_count = self.row_count();
        if index < row_count {
            (index, 0)
        } else {
            (row_count - 1, index - (row_count - 1))
        }
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn column_count(&self) -> usize {
        self.rows.first().map_or(0, |row| row.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = TransposingMatrices {
            row_count: 2,
            column_count: 2,
            before: Matrix {
                rows: vec![vec![1, 1], vec![6, 1]],
            },
            after: Matrix {
                rows: vec![vec![1, 6], vec![1, 1]],
            },
        };
        let actual = problem.solve()?;
        assert_eq!(true, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = TransposingMatrices {
            row_count: 2,
            column_count: 2,
            before: Matrix {
                rows: vec![vec![4, 4], vec![4, 5]],
            },
            after: Matrix {
                rows: vec![vec![5, 4], vec![4, 4]],
            },
        };
        let actual = problem.solve()?;
        assert_eq!(false, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let problem = TransposingMatrices {
            row_count: 3,
            column_count: 3,
            before: Matrix {
                rows: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            },
            after: Matrix {
                rows: vec![vec![1, 4, 7], vec![2, 5, 6], vec![3, 8, 9]],
            },
        };
        let actual = problem.solve()?;
        assert_eq!(true, actual);
        Ok(())
    }

    #[test]
    fn test_simple_negative() -> Result<()> {
        let problem = TransposingMatrices {
            row_count: 1,
            column_count: 1,
            before: Matrix {
                rows: vec![vec![1]],
            },
            after: Matrix {
                rows: vec![vec![2]],
            },
        };
        let actual = problem.solve()?;
        assert_eq!(false, actual);
        Ok(())
    }

    #[test]
    fn test_simple_positive() -> Result<()> {
        let problem = TransposingMatrices {
            row_count: 1,
            column_count: 1,
            before: Matrix {
                rows: vec![vec![1]],
            },
            after: Matrix {
                rows: vec![vec![1]],
            },
        };
        let actual = problem.solve()?;
        assert_eq!(true, actual);
        Ok(())
    }
}
