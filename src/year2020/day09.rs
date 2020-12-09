pub use crate::prelude::*;

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part1(a: &[i64]) -> i64 {
    let preamble = 25;

    for i in preamble..a.len() {
        let mut window = a[i - preamble..i].to_vec();
        window.sort_unstable();
        if find_sum2(&window, a[i]).is_none() {
            return a[i];
        }
    }
    
    -1
}

#[aoc(day9, part2)]
fn part2(a: &[i64]) -> i64 {
    let target = part1(a);
    let (i, j) = find_diff2(&running_sum(a), target).unwrap();
    a[i..j].iter().min().unwrap() + a[i..j].iter().max().unwrap()
}
