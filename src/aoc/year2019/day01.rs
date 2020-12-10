pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(a: &[i64]) -> i64 {
    a
        .iter()
        .map(|x| x / 3 - 2)
        .sum()
}

fn part2(a: &[i64]) -> i64 {
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
