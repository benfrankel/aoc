use crate::prelude::*;

pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part1(a: &[usize]) -> impl Debug {
    let nth = 2020;

    let mut prev = vec![None; nth];
    for i in 0..a.len() - 1 {
        prev[a[i]] = Some(i);
    }

    let mut num = a[a.len() - 1];
    for i in a.len() - 1..nth - 1 {
        let age = i - prev[num].unwrap_or(i);
        prev[num] = Some(i);
        num = age;
    }

    num
}

pub fn part2(a: &[usize]) -> impl Debug {
    let nth = 30000000;

    let mut prev = vec![None; nth];
    for i in 0..a.len() - 1 {
        prev[a[i]] = Some(i);
    }

    let mut num = a[a.len() - 1];
    for i in a.len() - 1..nth - 1 {
        let age = i - prev[num].unwrap_or(i);
        prev[num] = Some(i);
        num = age;
    }

    num
}
