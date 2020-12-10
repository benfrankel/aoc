pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn run(mut code: Vec<i64>, noun: i64, verb: i64) -> i64 {
    code[1] = noun;
    code[2] = verb;
    let mut ip = 0;
    while code[ip] != 99 {
        let a = code[ip + 1] as usize;
        let b = code[ip + 2] as usize;
        let c = code[ip + 3] as usize;
        code[c] = match code[ip] {
            1 => code[a] + code[b],
            2 => code[a] * code[b],
            _ => panic!("Unknown opcode."),
        };
        ip += 4;
    }
    code[0]
}

fn part1(a: &[i64]) -> i64 {
    run(a.to_vec(), 12, 2)
}

fn part2(a: &[i64]) -> i64 {
    let target = 19690720;
    
    for noun in 0..100 {
        for verb in 0..100 {
            if run(a.to_vec(), noun, verb) == target {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Couldn't find a solution.");
}
