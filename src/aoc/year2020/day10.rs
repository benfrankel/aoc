use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    let mut a: Vec<_> =
        input.lines().map(|line| line.parse().unwrap()).collect();
    a.push(0);
    a.push(a.iter().max().unwrap() + 3);
    a.sort_unstable();
    a
}

pub fn part1(a: &[i64]) -> impl Debug {
    [1, 3]
        .iter()
        .map(|diff| {
            a.windows(2)
                .filter(|window| window[1] - window[0] == *diff)
                .count()
        })
        .product::<usize>()
}

pub fn part2(a: &[i64]) -> impl Debug {
    let mut paths = vec![0i64; a.len()];
    paths[0] = 1;
    for i in 1..a.len() {
        for j in (0..i).rev() {
            if a[j] < a[i] - 3 {
                break;
            }
            paths[i] += paths[j];
        }
    }
    *paths.last().unwrap()
}
