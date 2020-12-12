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
    let (i, j) = find_window_sum(a, part1(a)).unwrap();
    a[i..j].iter().min().unwrap() + a[i..j].iter().max().unwrap()
}
