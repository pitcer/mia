use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

fn count_odd(a: &[i32]) -> usize {
    a.iter().filter(|i| **i % 2 == 1).count()
}

fn count_even(a: &[i32]) -> usize {
    a.iter().filter(|i| **i % 2 == 0).count()
}

fn partition_even_odd(a: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    a.into_iter().partition(|element| element % 2 == 0)
}

fn sum_mins(bound: i32, numbers: Vec<i32>) -> i32 {
    let mut heap = numbers.into_iter().map(Reverse).collect::<BinaryHeap<_>>();
    let mut count = 0;
    for _ in 0..bound {
        if let Some(Reverse(n)) = heap.pop() {
            count += n;
        } else {
            panic!("")
            // return count;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let lines = stdin.lines().collect::<io::Result<Vec<String>>>().unwrap();
    assert_eq!(lines.len(), 2);

    let n = lines[0].parse::<usize>().unwrap();
    let mut a = lines[1]
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect::<Vec<i32>>();

    assert_eq!(a.len(), n);

    let even = count_even(&a);
    let odd = count_odd(&a);
    assert_eq!(even + odd, n);

    let abs_diff = i32::abs(even as i32 - odd as i32);
    if abs_diff <= 1 {
        println!("0");
        return;
    }

    // a.sort_unstable();

    let (evens, odds) = partition_even_odd(a);
    if even > odd {
        let even_sum = sum_mins(even as i32 - (odd as i32 + 1), evens);
        println!("{even_sum}");
    } else {
        let odd_sum = sum_mins(odd as i32 - (even as i32 + 1), odds);
        println!("{odd_sum}");
    }
}
