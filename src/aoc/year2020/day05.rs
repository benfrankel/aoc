use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| binary_to_i64(line.replace("R", "B").chars(), 'B'))
        .collect()
}

pub fn part1(ids: &[i64]) -> i64 {
    *ids.iter().max().unwrap()
}

pub fn part2(ids: &[i64]) -> i64 {
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
