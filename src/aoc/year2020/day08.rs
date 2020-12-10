pub use crate::prelude::*;

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

#[derive(Clone, PartialEq, Eq)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

type Instruction = (Op, i64);

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (op, num) = line.split2(" ");
            let op = match op.as_str() {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                _ => panic!("Invalid op: {}", op),
            };
            let num = num.parse().unwrap();
            (op, num)
        })
        .collect()
}

fn part1(input: &[Instruction]) -> i64 {
    let mut seen = vec![false; input.len()];
    let mut pc = 0;
    let mut acc = 0;
    while !seen[pc as usize] {
        seen[pc as usize] = true;
        let (op, num) = input[pc as usize].clone();
        pc += 1;
        match op {
            Op::Nop => (),
            Op::Acc => acc += num,
            Op::Jmp => pc += num - 1,
        }
    }

    acc
}

fn part2(input: &[Instruction]) -> i64 {
    let mut graph: HashMap<_, Vec<_>> = hashmap!{};
    for (pc, (op, num)) in input.iter().enumerate() {
        let next = pc as i64 + match op {
            Op::Nop | Op::Acc => 1,
            Op::Jmp => *num,
        };
        graph.entry((0, next)).or_default().push((0, pc));
        graph.entry((1, next)).or_default().push((1, pc));

        if *op == Op::Jmp || *op == Op::Nop {
            let flipped_next = pc as i64 + match op {
                Op::Jmp | Op::Acc => 1,
                Op::Nop => *num,
            };
            graph.entry((1, flipped_next)).or_default().push((0, pc));
        }
    }

    let path = bfs(
        &(1, input.len()),
        |(depth, pc)| graph.entry((*depth, *pc as i64)).or_default().clone(),
        |(depth, pc)| *depth == 0 && *pc == 0,
    ).unwrap();

    let mut acc = 0;
    for (_, pc) in path.iter().skip(1) {
        let (op, num) = input[*pc as usize].clone();
        if let Op::Acc = op {
            acc += num;
        }
    }
    
    acc
}
