use crate::prelude::*;

pub fn parse(input: &str) -> (i64, Vec<(i64, i64)>) {
    let (start, ids) = input.split2("\n");
    let start: i64 = start.parse().unwrap();
    let ids = ids
        .trim()
        .split(",")
        .map(|id| id.parse::<i64>().ok())
        .enumerate()
        .filter_map(|(i, id)| id.map(|id| (i as i64, id)))
        .collect();
    (start, ids)
}

pub fn part1(input: &(i64, Vec<(i64, i64)>)) -> i64 {
    let (start, buses) = input;

    for time in *start.. {
        if let Some((_, id)) = buses
            .iter()
            .find(|(_, id)| time % id == 0) {
                return id * (time - *start);
        }
    }
    
    unreachable!()
}

pub fn part2(input: &(i64, Vec<(i64, i64)>)) -> i64 {
    let (start, buses) = input;

    let mut residues = vec![];
    let mut modulii = vec![];
    for (i, id) in buses {
        residues.push((id - (*i % id)) % id);
        modulii.push(*id);
    }

    let (mut time, modulus) = chinese_remainder(&residues, &modulii);
    while time < *start {
        time += modulus;
    }
    
    time
}
