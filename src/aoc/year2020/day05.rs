use crate::prelude::*;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| binary_to_i64(line.replace("R", "B").chars(), 'B'))
        .collect()
}

pub fn part1(a: &[i64]) -> impl Debug {
    *a.iter().max().unwrap()
}

pub fn part2(a: &[i64]) -> impl Debug {
    let lo = *a.iter().min().unwrap();
    let hi = *a.iter().max().unwrap();

    let mut seen = vec![false; (hi - lo + 1) as usize];
    for id in a {
        seen[(id - lo) as usize] = true;
    }
    for (i, seen) in seen.iter().enumerate() {
        if !seen {
            return i as i64 + lo;
        }
    }

    panic!("Couldn't find a missing seat.")
}
