use std::cmp::Ordering;

use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1(a: &[i64]) -> i64 {
    let preamble = 25;

    for i in preamble..a.len() {
        let mut window = a[i - preamble..i].to_vec();
        window.sort_unstable();
        if find_sum2(&window, a[i]).is_none() {
            return a[i];
        }
    }
    
    panic!("Couldn't find an appropriate window.")
}

pub fn part2(a: &[i64]) -> i64 {
    let target = part1(a);
    let mut i = 0;
    let mut j = 0;
    let mut run_sum = 0;
    while i < a.len() && j < a.len() {
        match run_sum.cmp(&target) {
            Ordering::Equal => break,
            Ordering::Less => {
                run_sum += a[j];
                j += 1;
            },
            Ordering::Greater => {
                run_sum -= a[i];
                i += 1;
            },
        }
    }
    a[i..j].iter().min().unwrap() + a[i..j].iter().max().unwrap()
}
