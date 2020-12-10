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
    a.push(0);
    a.push(a.iter().max().unwrap() + 3);
    a.sort_unstable();
    a
}

fn part1(a: &[i64]) -> i64 {
    [1, 3]
        .iter()
        .map(|diff| a
             .windows(2)
             .filter(|window| window[1] - window[0] == *diff)
             .count() as i64
        )
        .product()
}

fn part2(a: &[i64]) -> i64 {
    let mut paths = vec![0; a.len()];
    paths[0] = 1;
    for i in 1..a.len() {
        for j in (0..i).rev() {
            if a[j] < a[i] - 3 {
                break;
            }
            paths[i] += paths[j];
        }
    }
    paths[a.len() - 1]
}
