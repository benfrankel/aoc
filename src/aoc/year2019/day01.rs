use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1(a: &[i64]) -> i64 {
    a
        .iter()
        .map(|x| x / 3 - 2)
        .sum()
}

pub fn part2(a: &[i64]) -> i64 {
    a
        .iter()
        .cloned()
        .map(|mut x| {
            let mut total = 0;
            while x >= 6 {
                x /= 3;
                x -= 2;
                total += x;
            }
            total
        })
        .sum()
}
