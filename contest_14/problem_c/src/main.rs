#![allow(deprecated_where_clause_location)]

use std::cmp::Ordering;
use std::io::{Stdin, StdinLock};
use std::ops::{RangeBounds, RangeInclusive};
use std::str::{FromStr, SplitWhitespace};
use std::{io, str};

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

type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error>;

type Problem = (GoldenSnitch, HarryPotter);
type ProblemResult = Option<(f64, Point<f64>)>;

const POINT_RANGE: RangeInclusive<i16> = -(10i16.pow(4))..=10i16.pow(4);
const SPEED_RANGE: RangeInclusive<u16> = 1..=10u16.pow(4);

fn main() -> Result<()> {
    let stdin = io::stdin();
    let scanner = StdinScanner::from(stdin);
    let problem = scan(scanner)?;
    let result = solve(problem);
    match result {
        None => println!("NO"),
        Some((time, Point(x, y, z))) => {
            println!("YES");
            println!("{time}");
            println!("{x} {y} {z}")
        }
    }
    Ok(())
}

fn scan(mut scanner: impl Scanner) -> Result<(GoldenSnitch, HarryPotter)> {
    let coordinate_count = scanner.next_line()?.next_ranged(1..=10u16.pow(4))?;
    let snitch_positions = (0..=coordinate_count)
        .map(|_| Point::scan(&mut scanner))
        .collect::<Result<Vec<_>, _>>()?;
    let (potter_velocity, snitch_velocity) = {
        let mut line = scanner.next_line()?;
        let potter_velocity = Velocity::scan(&mut line)?;
        let snitch_velocity = Velocity::scan(&mut line)?;
        debug_assert!(potter_velocity >= snitch_velocity);
        (potter_velocity, snitch_velocity)
    };
    let potter_position = Point::scan(&mut scanner)?.into_floating();
    let snitch = GoldenSnitch::new(snitch_positions, snitch_velocity);
    let potter = HarryPotter::new(potter_position, potter_velocity);
    Ok((snitch, potter))
}

fn solve((snitch, potter): Problem) -> ProblemResult {
    debug_assert!(snitch.positions.len() >= 2);

    if potter.position == snitch.positions[0].into_floating() {
        return Some((0.0, potter.position));
    }

    let mut time = 0.0;
    let (first, second) = snitch.positions.windows(2).find_map(|positions| {
        let first = positions[0].into_floating();
        let second = positions[1].into_floating();
        let time_to_travel = snitch.time_to_travel(first, second);
        let snitch_time = time + time_to_travel;
        if snitch_time >= potter.time_to_travel(second) {
            return Some((first, second));
        }
        time = snitch_time;
        None
    })?;

    let line = Line::new(first, second);
    let mut left = 0.0;
    let mut right = 1.0;
    for _ in 0..200 {
        let middle = (left + right) * 0.5;
        let point = line.point(middle);
        let snitch_time = time + snitch.time_to_travel(first, point);
        let potter_time = potter.time_to_travel(point);
        match snitch_time.total_cmp(&potter_time) {
            Ordering::Less => left = middle,
            Ordering::Equal => break,
            Ordering::Greater => right = middle,
        }
    }
    let middle = (left + right) * 0.5;
    let point = line.point(middle);
    let time = potter.time_to_travel(point);

    Some((time, point))
}

struct GoldenSnitch {
    positions: Vec<Point<i16>>,
    velocity: Velocity,
}

impl GoldenSnitch {
    pub fn new(positions: Vec<Point<i16>>, velocity: Velocity) -> Self {
        Self {
            positions,
            velocity,
        }
    }

    pub fn time_to_travel(&self, source: Point<f64>, destination: Point<f64>) -> f64 {
        source.time_to_travel(destination, self.velocity)
    }
}

struct HarryPotter {
    position: Point<f64>,
    velocity: Velocity,
}

impl HarryPotter {
    pub fn new(position: Point<f64>, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    pub fn time_to_travel(&self, destination: Point<f64>) -> f64 {
        self.position.time_to_travel(destination, self.velocity)
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct Velocity(f64);

impl Velocity {
    pub fn scan(line: &mut impl LineScanner) -> Result<Self> {
        let speed = line.next_ranged(SPEED_RANGE)?;
        Ok(Self(speed as f64))
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct Point<T>(T, T, T);

impl Point<i16> {
    pub fn scan(scanner: &mut impl Scanner) -> Result<Self> {
        let mut line = scanner.next_line()?;
        let x = line.next_ranged(POINT_RANGE)?;
        let y = line.next_ranged(POINT_RANGE)?;
        let z = line.next_ranged(POINT_RANGE)?;
        Ok(Self(x, y, z))
    }

    pub fn into_floating(self) -> Point<f64> {
        Point(self.0 as f64, self.1 as f64, self.2 as f64)
    }
}

impl Point<f64> {
    pub fn distance(&self, other: Point<f64>) -> f64 {
        f64::sqrt(
            (self.0 - other.0).powi(2) + (self.1 - other.1).powi(2) + (self.2 - other.2).powi(2),
        )
    }

    pub fn time_to_travel(&self, destination: Point<f64>, speed: Velocity) -> f64 {
        self.distance(destination) / speed.0
    }
}

#[derive(Debug)]
struct Line(Point<f64>, Point<f64>);

impl Line {
    pub fn new(first: Point<f64>, second: Point<f64>) -> Self {
        Self(first, second)
    }

    pub fn point(&self, time: f64) -> Point<f64> {
        debug_assert!((0.0..=1.0).contains(&time));

        let time_complement = 1.0 - time;
        let x = time_complement * self.0 .0 + time * self.1 .0;
        let y = time_complement * self.0 .1 + time * self.1 .1;
        let z = time_complement * self.0 .2 + time * self.1 .2;
        Point(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let line = Line::new(Point(0.0, 0.0, 0.0), Point(1.0, 1.0, 0.0));
        assert_eq!(Point(0.0, 0.0, 0.0), line.point(0.0));
        assert_eq!(Point(1.0, 1.0, 0.0), line.point(1.0));
        assert_eq!(Point(0.5, 0.5, 0.0), line.point(0.5));
        assert_eq!(Point(0.75, 0.75, 0.0), line.point(0.75));
        assert_eq!(Point(0.25, 0.25, 0.0), line.point(0.25));
    }

    #[test]
    fn test_example_1() -> Result<()> {
        let scanner = StringScanner::from(
            r#"4
            0 0 0
            0 10 0
            10 10 0
            10 0 0
            0 0 0
            1 1
            5 5 25"#,
        );
        let problem = scan(scanner)?;
        let actual = solve(problem);
        assert_eq!(Some((25.5, Point(10.0, 4.500000000000002, 0.0))), actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let scanner = StringScanner::from(
            r#"4
            0 0 0
            0 10 0
            10 10 0
            10 0 0
            0 0 0
            1 1
            5 5 50"#,
        );
        let problem = scan(scanner)?;
        let actual = solve(problem);
        assert_eq!(None, actual);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let scanner = StringScanner::from(
            r#"1
            1 2 3
            4 5 6
            20 10
            1 2 3"#,
        );
        let problem = scan(scanner)?;
        let actual = solve(problem);
        assert_eq!(Some((0.0, Point(1.0, 2.0, 3.0))), actual);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let scanner = StringScanner::from(
            r#"1
            0 0 0
            2 0 0
            1 1
            1 1 0"#,
        );
        let problem = scan(scanner)?;
        let actual = solve(problem);
        assert_eq!(Some((1.0, Point(1.0, 0.0, 0.0))), actual);
        Ok(())
    }

    #[test]
    fn test_example_5() -> Result<()> {
        let scanner = StringScanner::from(
            r#"1
            0 0 0
            3 0 0
            1 1
            3 0 0"#,
        );
        let problem = scan(scanner)?;
        let actual = solve(problem);
        assert_eq!(Some((1.5, Point(1.5, 0.0, 0.0))), actual);
        Ok(())
    }

    #[test]
    fn test_example_6() -> Result<()> {
        let scanner = StringScanner::from(
            r#"1
            0 0 0
            2 0 0
            1 1
            1 0 0"#,
        );
        let problem = scan(scanner)?;
        let actual = solve(problem);
        assert_eq!(Some((0.5, Point(0.5, 0.0, 0.0))), actual);
        Ok(())
    }
}
