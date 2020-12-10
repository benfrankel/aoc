pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<i64> {
    let mut a: Vec<i64> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    a.sort_unstable();
    a
}

fn part1(a: &[i64]) -> i64 {
    let (i, j) = find_sum2(&a, 2020).unwrap();
    a[i] * a[j]
}

fn part2(a: &[i64]) -> i64 {
    let (i, j, k) = find_sum3(&a, 2020).unwrap();
    a[i] * a[j] * a[k]
}
