use std::collections::HashSet;
use std::io;
use std::io::{BufRead, Lines};

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
    parse_next!(lines => let query_count: usize = 1..=100);
    let ping_pong = PingPong::new(query_count)?;
    ping_pong.solve(lines)?;
    Ok(())
}

type IntervalId = usize;

struct PingPong {
    intervals: Vec<Interval>,
    edges: Vec<HashSet<IntervalId>>,
}

impl PingPong {
    fn new(query_count: usize) -> Result<Self> {
        Ok(Self {
            intervals: Vec::with_capacity(query_count),
            edges: Vec::with_capacity(query_count),
        })
    }

    fn solve(mut self, lines: Lines<impl BufRead>) -> Result<()> {
        for line in lines {
            let line = line?;
            parse_next! { line ->
                let query_type: u8 = 1..=2;
                let first: i32 = -(10i32.pow(9))..=10i32.pow(9);
                let second: i32 = -(10i32.pow(9))..=10i32.pow(9)
            }
            match query_type {
                1 => self.handle_new_interval(first, second)?,
                2 => self.handle_path_exists(first as IntervalId - 1, second as IntervalId - 1)?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn handle_new_interval(&mut self, lower: i32, upper: i32) -> Result<()> {
        debug_assert_eq!(self.intervals.len(), self.edges.len());

        let interval = Interval::new(lower, upper);
        let id = self.intervals.len();
        let mut paths = HashSet::new();
        for (other_id, other) in self.intervals.iter().enumerate() {
            if other.contains(lower) || other.contains(upper) {
                paths.insert(other_id);
            }

            if interval.contains(other.lower()) || interval.contains(other.upper()) {
                self.edges[other_id].insert(id);
            }
        }

        self.intervals.push(interval);
        self.edges.push(paths);

        Ok(())
    }

    fn handle_path_exists(&mut self, from_id: IntervalId, to_id: IntervalId) -> Result<()> {
        let mut visited = BitSet::new();
        let mut stack = Vec::with_capacity(self.intervals.len());

        stack.push(from_id);
        while let Some(id) = stack.pop() {
            visited.insert(id);
            if id == to_id {
                println!("YES");
                return Ok(());
            }

            for neighbour_id in &self.edges[id] {
                if !visited.contains(*neighbour_id) {
                    stack.push(*neighbour_id);
                }
            }
        }

        println!("NO");
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Interval(i32, i32);

impl Interval {
    fn new(lower: i32, upper: i32) -> Self {
        Self(lower, upper)
    }

    fn contains(&self, number: i32) -> bool {
        self.0 < number && number < self.1
    }

    fn lower(&self) -> i32 {
        self.0
    }

    fn upper(&self) -> i32 {
        self.1
    }
}

#[derive(Copy, Clone)]
struct BitSet(u128);

impl BitSet {
    fn new() -> Self {
        Self(0)
    }

    fn insert(&mut self, index: usize) {
        debug_assert!(index < 100);
        self.0 |= 1u128 << index;
    }

    fn contains(&self, index: usize) -> bool {
        debug_assert!(index < 100);
        (self.0 >> index) & 1u128 == 1
    }
}
