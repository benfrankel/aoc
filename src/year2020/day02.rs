pub use crate::prelude::*;

type Case = (usize, usize, char, String);

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Case> {
    let mut rules = vec![];
    
    for line in input.lines() {
        let (rule, password) = line.split2(": ");
        let (range, ch) = rule.split2(" ");
        let ch = ch.chars().next().unwrap();
        let (lo, hi) = range.split2("-");
        let lo = lo.parse().unwrap();
        let hi = hi.parse().unwrap();

        rules.push((lo, hi, ch, password));
    }
    
    rules
}

#[aoc(day2, part1)]
fn part1(input: &[Case]) -> i64 {
    let mut count = 0;
    for (lo, hi, ch, password) in input {
        if (lo..=hi).contains(&&password.chars().filter(|c| c == ch).count()) {
            count += 1;
        }
    }
    count
}

#[aoc(day2, part2)]
fn part2(input: &[Case]) -> i64 {
    let mut count = 0;
    for (lo, hi, ch, password) in input {
        let c1 = password.chars().nth(lo - 1).unwrap();
        let c2 = password.chars().nth(hi - 1).unwrap();
        if (c1 == *ch || c2 == *ch) && c1 != c2 {
            count += 1;
        }
    }
    count
}
