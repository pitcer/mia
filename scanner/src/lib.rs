#![allow(deprecated_where_clause_location)]

use std::io::{Stdin, StdinLock};
use std::ops::RangeBounds;
use std::str::{FromStr, SplitWhitespace};
use std::{io, str};

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error>;

trait LineScanner {
    fn next_ranged<U>(&mut self, range: impl RangeBounds<U>) -> Result<U>
    where
        U: PartialOrd + FromStr,
        U::Err: std::error::Error + 'static;
}

trait Scanner {
    type LineScanner<'a>: LineScanner
    where
        Self: 'a;

    fn next_line(&mut self) -> Result<Self::LineScanner<'_>>;
}

struct StringScanner<'a> {
    input: str::Lines<'a>,
}

impl<'a> StringScanner<'a> {
    fn new(input: str::Lines<'a>) -> Self {
        Self { input }
    }
}

impl<'b> Scanner for StringScanner<'b> {
    type LineScanner<'a>
    where
        Self: 'a,
    = LineSplitScanner<'a>;

    fn next_line(&mut self) -> Result<Self::LineScanner<'_>> {
        let next_line = self.input.next().ok_or("Missing next line")?;
        let line = next_line.split_whitespace();
        Ok(LineSplitScanner { line })
    }
}

impl<'a> From<&'a str> for StringScanner<'a> {
    fn from(value: &'a str) -> Self {
        let lines = value.lines();
        Self::new(lines)
    }
}

impl<'a> Drop for StringScanner<'a> {
    fn drop(&mut self) {
        debug_assert!(self.input.next().is_none());
    }
}

struct StdinScanner {
    input: io::Lines<StdinLock<'static>>,
    current_line: Option<String>,
}

impl StdinScanner {
    fn new(input: io::Lines<StdinLock<'static>>) -> Self {
        Self {
            input,
            current_line: None,
        }
    }
}

impl Scanner for StdinScanner {
    type LineScanner<'a>
    where
        Self: 'a,
    = LineSplitScanner<'a>;

    fn next_line(&mut self) -> Result<Self::LineScanner<'_>> {
        let next_line = self.input.next().ok_or("Missing next line")??;
        let line = self.current_line.insert(next_line);
        let line = line.split_whitespace();
        Ok(LineSplitScanner { line })
    }
}

impl From<Stdin> for StdinScanner {
    fn from(value: Stdin) -> Self {
        let lines = value.lines();
        Self::new(lines)
    }
}

impl Drop for StdinScanner {
    fn drop(&mut self) {
        debug_assert!(self.input.next().is_none());
    }
}

struct LineSplitScanner<'a> {
    line: SplitWhitespace<'a>,
}

impl<'a> LineScanner for LineSplitScanner<'a> {
    fn next_ranged<U>(&mut self, range: impl RangeBounds<U>) -> Result<U>
    where
        U: PartialOrd + FromStr,
        U::Err: std::error::Error + 'static,
    {
        let item = self.line.next().ok_or("Missing next item")?;
        let parsed = item.parse::<U>()?;
        debug_assert!(range.contains(&parsed));
        Ok(parsed)
    }
}

impl<'a> Drop for LineSplitScanner<'a> {
    fn drop(&mut self) {
        debug_assert!(self.line.next().is_none());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() -> Result<()> {
        let mut scanner = StringScanner::from(
            r#"1 2 3
            1"#,
        );
        let (x, y, z) = {
            let mut line = scanner.next_line()?;
            let x = line.next_ranged(0..10)?;
            let y = line.next_ranged(0..10)?;
            let z = line.next_ranged(0..10)?;
            (x, y, z)
        };
        let a = scanner.next_line()?.next_ranged(0..10)?;
        assert_eq!(((1, 2, 3), 1), ((x, y, z), a));
        Ok(())
    }
}
