use std::array::TryFromSliceError;
use std::cmp::Reverse;
use std::io;
use std::io::BufRead;

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

trait Problem<T>: Sized {
    fn from_reader(reader: impl BufRead) -> Result<Self>;

    fn solve(self) -> Result<T>;
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let problem = Tape::from_reader(stdin)?;
    let result = problem.solve()?;
    println!("{result}");
    Ok(())
}

struct Tape {
    stick: Stick,
    tape_pieces: u32,
}

struct Stick {
    segment_count: u32,
    broken_segments: Vec<u32>,
}

impl Problem<u32> for Tape {
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
        let second_line = lines.get(1).ok_or("Missing second line")?;

        let first_line = split_line(first_line)?;
        let broken_segment_count = *first_line.get(0).ok_or("Missing broken segment count")?;
        let segment_count = *first_line.get(1).ok_or("Missing segment count")?;
        let tape_pieces = *first_line.get(2).ok_or("Missing tape pieces")?;
        let broken_segments = split_line(second_line)?;
        assert_eq!(broken_segments.len(), broken_segment_count as usize);

        let stick = Stick {
            segment_count,
            broken_segments,
        };
        let tape = Tape { stick, tape_pieces };

        assert!((1..=100_000).contains(&broken_segment_count));
        assert!((broken_segment_count..=1_000_000_000).contains(&tape.stick.segment_count));
        assert!((1..=broken_segment_count).contains(&tape.tape_pieces));
        for segment in &tape.stick.broken_segments {
            assert!((1..=tape.stick.segment_count).contains(segment))
        }

        Ok(tape)
    }

    /// Returns the minimum total length of the tape pieces.
    ///
    /// Idea:
    /// Let k = tape_pieces.
    /// First since k >= 1, put one tape piece from first broken segment to the last one.
    /// Then cut tape on longest gaps k - 1 times to get k tape pieces in total.
    /// Then sum the length of all tape pieces.
    fn solve(self) -> Result<u32> {
        let broken_segments = &self.stick.broken_segments;
        let first_broken_segment = broken_segments.first().ok_or("broken_segments are empty")?;
        let last_broken_segment = broken_segments.last().ok_or("broken_segments are empty")?;
        let tape_length = last_broken_segment - first_broken_segment + 1;

        let mut broken_segments_distances = broken_segments
            .windows(2)
            .map(|window| {
                let [first, second]: [u32; 2] = window.try_into()?;
                Ok(second - first)
            })
            .collect::<Result<Vec<_>, TryFromSliceError>>()?;

        broken_segments_distances.sort_unstable_by_key(|distance| Reverse(*distance));
        let removed_tape_pieces_sum = broken_segments_distances
            .into_iter()
            .take(self.tape_pieces as usize - 1)
            .map(|distance| distance - 1)
            .sum::<u32>();

        Ok(tape_length - removed_tape_pieces_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let problem = Tape {
            stick: Stick {
                segment_count: 100,
                broken_segments: vec![20, 30, 75, 80],
            },
            tape_pieces: 2,
        };
        let actual = problem.solve()?;
        assert_eq!(17, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let problem = Tape {
            stick: Stick {
                segment_count: 100,
                broken_segments: vec![1, 2, 4, 60, 87],
            },
            tape_pieces: 3,
        };
        let actual = problem.solve()?;
        assert_eq!(6, actual);
        Ok(())
    }

    #[test]
    fn test_simple() -> Result<()> {
        let problem = Tape {
            stick: Stick {
                segment_count: 100,
                broken_segments: vec![60],
            },
            tape_pieces: 1,
        };
        let actual = problem.solve()?;
        assert_eq!(1, actual);
        Ok(())
    }
}
