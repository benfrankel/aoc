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
    let preamble = 25;

    for i in preamble..a.len() {
        let mut window = a[i - preamble..i].to_vec();
        window.sort_unstable();
        if find_sum2(&window, a[i]).is_none() {
            return a[i];
        }
    }
    
    panic!("Couldn't find an appropriate window.")
}

fn part2(a: &[i64]) -> i64 {
    let target = part1(a);
    let (i, j) = find_diff2(&running_sum(a), target).unwrap();
    a[i..j].iter().min().unwrap() + a[i..j].iter().max().unwrap()
}
