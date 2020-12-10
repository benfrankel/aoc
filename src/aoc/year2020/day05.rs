pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| binary_to_i64(line.replace("R", "B").chars(), 'B'))
        .collect()
}

fn part1(ids: &[i64]) -> i64 {
    *ids.iter().max().unwrap()
}

fn part2(ids: &[i64]) -> i64 {
    let lo = *ids.iter().min().unwrap();
    let hi = *ids.iter().max().unwrap();

    let mut seen = vec![false; (hi - lo + 1) as usize];
    for id in ids {
        seen[(id - lo) as usize] = true;
    }
    for (i, seen) in seen.iter().enumerate() {
        if !seen {
            return i as i64 + lo;
        }
    }

    panic!("Couldn't find a missing seat.")
}
