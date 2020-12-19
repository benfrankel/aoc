use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Mul,
}

pub fn parse(input: &str) -> Vec<Vec<Token<Op>>> {
    input
        .lines()
        .map(|line| {
            line.replace(' ', "")
                .chars()
                .map(|c| match c {
                    '+' => Token::Op(Op::Add),
                    '*' => Token::Op(Op::Mul),
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    x if x.is_ascii_digit() => {
                        Token::Num(x.to_digit(10).unwrap() as i64)
                    }
                    x => panic!("Unknown token: {}", x),
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &[Vec<Token<Op>>]) -> impl Debug {
    let op_info = hashmap! {
        Op::Add => OpInfo {
            precedence: 0,
            eval: |a, b| a + b,
        },
        Op::Mul => OpInfo {
            precedence: 0,
            eval: |a, b| a * b,
        },
    };
    input.iter().map(|expr| eval(expr, &op_info)).sum::<i64>()
}

pub fn part2(input: &[Vec<Token<Op>>]) -> impl Debug {
    let op_info = hashmap! {
        Op::Add => OpInfo {
            precedence: 1,
            eval: |a, b| a + b,
        },
        Op::Mul => OpInfo {
            precedence: 0,
            eval: |a, b| a * b,
        },
    };
    input.iter().map(|expr| eval(expr, &op_info)).sum::<i64>()
}
