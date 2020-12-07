pub use crate::prelude::*;

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<(String, Vec<(i64, String)>)> {
    let mut rules = vec![];
    
    for line in input.lines() {
        let (outside, inside) = line[..line.len() - 1]
            .replace(" bags", "")
            .replace(" bag", "")
            .split2(" contain ");
        let inside = if inside == "no other" {
            vec![]
        } else {
            inside
                .split(", ")
                .map(|s| {
                    let (num, bag) = s.split2(" ");
                    let num: i64 = num.parse().unwrap();
                    (num, bag)
                })
                .collect()
        };
        rules.push((outside, inside))
    }
    
    rules
}

#[aoc(day7, part1)]
fn part1(rules: &[(String, Vec<(i64, String)>)]) -> i64 {
    let mut contains = hashmap!{};
    for (outside, inside) in rules.iter().cloned() {
        for (_, bag) in inside {
            contains.entry(bag).or_insert(vec![]).push(outside.clone());
        }
    }

    let start = "shiny gold";
    let mut seen = hashset!{};
    let mut queue = vec![];
    queue.push(start.to_string());
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        for bag in contains.entry(next).or_insert(vec![]).iter() {
            if seen.contains(bag) {
                continue;
            }
            seen.insert(bag.clone());
            queue.push(bag.clone());
        }
    }

    seen.len() as _
}

#[aoc(day7, part2)]
fn part2(rules: &[(String, Vec<(i64, String)>)]) -> i64 {
    let mut contains = hashmap!{};
    for (outside, inside) in rules.iter().cloned() {
        for (num, bag) in inside {
            contains.entry(outside.clone()).or_insert(vec![]).push((num, bag));
        }
    }

    let mut count = 0;
    let start = "shiny gold";
    let mut queue = vec![];
    queue.push((1, start.to_string()));
    while !queue.is_empty() {
        let (next_num, next_bag) = queue.pop().unwrap();
        for (num, bag) in contains.entry(next_bag.clone()).or_insert(vec![]).iter() {
            queue.push((next_num * num, bag.clone()));
            count += next_num * num;
        }
    }

    count
}
