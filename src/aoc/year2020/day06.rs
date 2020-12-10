pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

type Group = Vec<HashSet<char>>;

fn parse(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|group| group
             .lines()
             .map(|line| line.chars().collect())
             .collect()
        )
        .collect()
}

fn part1(input: &[Group]) -> i64 {
    input
        .iter()
        .map(|group| group
             .iter()
             .skip(1)
             .fold(group[0].clone(), |mut acc, person| {
                 acc.extend(person);
                 acc
             })
             .len()
        )
        .sum::<usize>() as _
}

fn part2(input: &[Group]) -> i64 {
    input
        .iter()
        .map(|group| group
             .iter()
             .skip(1)
             .fold(group[0].clone(), |mut acc, person| {
                 acc.retain(|c| person.contains(c));
                 acc
             })
             .len()
        )
        .sum::<usize>() as _
}
