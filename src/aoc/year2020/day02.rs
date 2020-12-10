pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

type Case = (usize, usize, char, String);

fn parse(input: &str) -> Vec<Case> {
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

fn part1(cases: &[Case]) -> i64 {
    let mut count = 0;
    for (lo, hi, ch, password) in cases {
        if (lo..=hi).contains(&&password.chars().filter(|c| c == ch).count()) {
            count += 1;
        }
    }
    count
}

fn part2(cases: &[Case]) -> i64 {
    let mut count = 0;
    for (lo, hi, ch, password) in cases {
        let c1 = password.chars().nth(lo - 1).unwrap();
        let c2 = password.chars().nth(hi - 1).unwrap();
        if (c1 == *ch || c2 == *ch) && c1 != c2 {
            count += 1;
        }
    }
    count
}
