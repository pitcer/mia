use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;

fn count_odd(a: &Vec<i32>) -> usize {
    a.iter().filter(|i| **i % 2 == 1).count()
}

fn count_even(a: &Vec<i32>) -> usize {
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
            return count;
        }
    }
    count
}

fn main() {
    let mut a_string = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    stdin.read_to_string(&mut a_string).unwrap();
    let lines = a_string.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 2);
    let n = lines[0].parse::<usize>().unwrap();

    let a = lines[1]
        .split_whitespace()
        .map(|i| i.parse().unwrap())
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

    let (odds, evens) = partition_even_odd(a);
    let odd_sum = sum_mins(abs_diff - 1, odds);
    let even_sum = sum_mins(abs_diff - 1, evens);

    if odd_sum == 0 {
        println!("{even_sum}");
        return;
    }

    if even_sum == 0 {
        println!("{odd_sum}");
        return;
    }
    let min_sum = std::cmp::min(odd_sum, even_sum);
    println!("{min_sum}");
}
