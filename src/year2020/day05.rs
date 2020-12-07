pub use crate::prelude::*;

#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| binary_to_i64(line.replace("R", "B").chars(), 'B'))
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[i64]) -> i64 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[i64]) -> i64 {
    let lo = *input.iter().min().unwrap();
    let hi = *input.iter().max().unwrap();

    let mut seen = vec![false; (hi - lo + 1) as usize];
    for id in input {
        seen[(id - lo) as usize] = true;
    }
    for (i, seen) in seen.iter().enumerate() {
        if !seen {
            return i as i64 + lo;
        }
    }

    panic!("Couldn't find a missing seat.")
}
