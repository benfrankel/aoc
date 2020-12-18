use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Nop,
    Acc,
    Jmp,
}

type Instruction = (Op, i64);

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (op, num) = line.split2(" ");
            let op = match op.as_str() {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                _ => panic!("Unknown op: {}", op),
            };
            let num = num.parse().unwrap();
            (op, num)
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> impl Debug {
    let mut seen = vec![false; input.len()];
    let mut ip = 0;
    let mut acc = 0;
    while !seen[ip as usize] {
        seen[ip as usize] = true;
        let (op, num) = input[ip as usize].clone();
        ip += 1;
        match op {
            Op::Nop => (),
            Op::Acc => acc += num,
            Op::Jmp => ip += num - 1,
        }
    }

    acc
}

pub fn part2(input: &[Instruction]) -> impl Debug {
    let mut graph: HashMap<_, Vec<_>> = hashmap! {};
    for (ip, (op, num)) in input.iter().enumerate() {
        let next = ip as i64
            + match op {
                Op::Nop | Op::Acc => 1,
                Op::Jmp => *num,
            };
        graph.entry((0, next)).or_default().push((0, ip));
        graph.entry((1, next)).or_default().push((1, ip));

        if *op == Op::Jmp || *op == Op::Nop {
            let flipped_next = ip as i64
                + match op {
                    Op::Jmp | Op::Acc => 1,
                    Op::Nop => *num,
                };
            graph.entry((1, flipped_next)).or_default().push((0, ip));
        }
    }

    let path = bfs(
        &(1, input.len()),
        |(depth, ip)| graph.entry((*depth, *ip as i64)).or_default().clone(),
        |(depth, ip)| *depth == 0 && *ip == 0,
    )
    .unwrap();

    let mut acc = 0;
    for (_, ip) in path.iter().skip(1) {
        let (op, num) = input[*ip as usize].clone();
        if let Op::Acc = op {
            acc += num;
        }
    }

    acc
}
