use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    let mut a: Vec<_> =
        input.lines().map(|line| line.parse().unwrap()).collect();
    a.sort_unstable();
    a
}

pub fn part1(a: &[i64]) -> impl Debug {
    let (i, j) = find_sum2(&a, 2020).unwrap();
    a[i] * a[j]
}

pub fn part2(a: &[i64]) -> impl Debug {
    let (i, j, k) = find_sum3(&a, 2020).unwrap();
    a[i] * a[j] * a[k]
}
