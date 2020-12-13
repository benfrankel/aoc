use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1(a: &[i64]) -> i64 {
    let preamble = 25;

    let mut window = a[0..preamble].to_vec();
    window.sort_unstable();
    for i in preamble..a.len() {
        if find_sum2(&window, a[i]).is_none() {
            return a[i];
        }
        let idx = window.binary_search(&a[i - preamble]).unwrap_or_else(|x| x);
        window.remove(idx);
        let idx = window.binary_search(&a[i]).unwrap_or_else(|x| x);
        window.insert(idx, a[i]);
    }
    
    panic!("Couldn't find an appropriate window.")
}

pub fn part2(a: &[i64]) -> i64 {
    let (i, j) = find_window_sum(a, part1(a)).unwrap();
    a[i..j].iter().min().unwrap() + a[i..j].iter().max().unwrap()
}
