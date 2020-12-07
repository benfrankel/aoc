pub use crate::prelude::*;

type Group = Vec<HashSet<char>>;

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|group| group
             .lines()
             .map(|line| line.chars().collect())
             .collect()
        )
        .collect()
}

#[aoc(day6, part1)]
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

#[aoc(day6, part2)]
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
