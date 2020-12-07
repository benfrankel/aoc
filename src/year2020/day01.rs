pub use crate::prelude::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<i64> {
    let mut a: Vec<i64> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    a.sort_unstable();
    a
}

#[aoc(day1, part1)]
fn part1(a: &[i64]) -> i64 {
    let (i, j) = find_sum2(a, 2020).unwrap();
    a[i] * a[j]
}

#[aoc(day1, part2)]
fn part2(a: &[i64]) -> i64 {
    let (i, j, k) = find_sum3(a, 2020).unwrap();
    a[i] * a[j] * a[k]
}
