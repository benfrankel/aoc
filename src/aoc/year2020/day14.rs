use crate::prelude::*;

#[derive(Debug)]
pub enum Command {
    Mask { zero: u64, one: u64, x: u64 },
    Write { address: u64, value: u64 },
}

pub fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| match line.chars().skip(1).next().unwrap() {
            'a' => {
                let (_, mask) = line.split2(" = ");
                Command::Mask {
                    zero: binary_to_i64(mask.chars(), '0') as u64,
                    one: binary_to_i64(mask.chars(), '1') as u64,
                    x: binary_to_i64(mask.chars(), 'X') as u64,
                }
            }
            'e' => {
                let (address, value) = line[4..].split2("] = ");
                Command::Write {
                    address: address.parse().unwrap(),
                    value: value.parse().unwrap(),
                }
            }
            _ => panic!("Unknown command: {}.", line),
        })
        .collect()
}

pub fn part1(input: &[Command]) -> impl Debug {
    let mut zero_mask = 0;
    let mut one_mask = 0;
    let mut mem: HashMap<u64, _> = hashmap! {};

    for command in input {
        match command {
            Command::Mask { zero, one, x: _ } => {
                zero_mask = *zero;
                one_mask = *one;
            }
            Command::Write { address, value } => {
                mem.insert(*address, value & !zero_mask | one_mask);
            }
        }
    }

    mem.values().cloned().sum::<u64>()
}

pub fn part2(input: &[Command]) -> impl Debug {
    let mut one_mask = 0;
    let mut x_mask = 0;
    let mut mem: HashMap<u64, _> = hashmap! {};

    for command in input {
        match command {
            Command::Mask { zero: _, one, x } => {
                one_mask = *one;
                x_mask = *x;
            }
            Command::Write { address, value } => {
                for float in 0..(1 << x_mask.count_ones()) {
                    let mut flip_mask = 0;
                    for (i, bit) in bit_positions(x_mask).iter().enumerate() {
                        flip_mask |= ((float >> i) & 1) << bit;
                    }
                    mem.insert((*address | one_mask) ^ flip_mask, value);
                }
            }
        }
    }

    mem.values().cloned().sum::<u64>()
}
