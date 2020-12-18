use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_special_number(a: &[i64], preamble: usize) -> Option<i64> {
    let mut window = a[0..preamble].to_vec();
    window.sort_unstable();
    for i in preamble..a.len() {
        if find_sum2(&window, a[i]).is_none() {
            return Some(a[i]);
        }
        let idx = window.binary_search(&a[i - preamble]).unwrap_or_else(|x| x);
        window.remove(idx);
        let idx = window.binary_search(&a[i]).unwrap_or_else(|x| x);
        window.insert(idx, a[i]);
    }

    None
}

pub fn part1(a: &[i64]) -> impl Debug {
    find_special_number(a, 25)
}

pub fn part2(a: &[i64]) -> impl Debug {
    let target = find_special_number(a, 25).unwrap();
    let (i, j) = find_window_sum(a, target).unwrap();
    a[i..j].iter().min().unwrap() + a[i..j].iter().max().unwrap()
}
