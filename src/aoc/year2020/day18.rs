use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Mul,
    LParen,
    RParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Op(Op),
    Num(i64),
}

pub fn parse(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| {
            line.replace(" ", "")
                .chars()
                .map(|c| match c {
                    '+' => Token::Op(Op::Add),
                    '*' => Token::Op(Op::Mul),
                    '(' => Token::Op(Op::LParen),
                    ')' => Token::Op(Op::RParen),
                    x if x.is_ascii_digit() => {
                        Token::Num(x.to_digit(10).unwrap() as i64)
                    }
                    x => panic!("Unknown token: {}", x),
                })
                .collect()
        })
        .collect()
}

fn eval(expr: &[Token], precedence_map: &HashMap<Op, i64>) -> i64 {
    let mut num_stack = vec![];
    let mut op_stack = vec![];

    fn acc(
        num_stack: &mut Vec<i64>,
        op_stack: &mut Vec<Op>,
        precedence_map: &HashMap<Op, i64>,
        precedence: i64,
    ) {
        while num_stack.len() >= 2
            && !op_stack.is_empty()
            && precedence_map[&op_stack[op_stack.len() - 1]] >= precedence
        {
            let op = op_stack.pop().unwrap();
            let b = num_stack.pop().unwrap();
            let a = num_stack.pop().unwrap();
            match op {
                Op::Add => num_stack.push(a + b),
                Op::Mul => num_stack.push(a * b),
                _ => panic!("Unexpected operator: {:?}", op),
            }
        }
    };

    for &token in std::iter::once(&Token::Op(Op::LParen))
        .chain(expr)
        .chain(std::iter::once(&Token::Op(Op::RParen)))
    {
        match token {
            Token::Num(x) => num_stack.push(x),
            Token::Op(op @ Op::LParen) => op_stack.push(op),
            Token::Op(Op::RParen) => {
                acc(&mut num_stack, &mut op_stack, &precedence_map, -1);
                op_stack.pop();
            }
            Token::Op(op) => {
                acc(
                    &mut num_stack,
                    &mut op_stack,
                    &precedence_map,
                    precedence_map[&op],
                );
                op_stack.push(op);
            }
        }
    }

    num_stack[0]
}

pub fn part1(input: &[Vec<Token>]) -> impl Debug {
    let precedence_map = hashmap! {
        Op::Add => 0,
        Op::Mul => 0,
        Op::LParen => -2,
        Op::RParen => -1,
    };
    input
        .iter()
        .map(|expr| eval(expr, &precedence_map))
        .sum::<i64>()
}

pub fn part2(input: &[Vec<Token>]) -> impl Debug {
    let precedence_map = hashmap! {
        Op::Add => 1,
        Op::Mul => 0,
        Op::LParen => -2,
        Op::RParen => -1,
    };
    input
        .iter()
        .map(|expr| eval(expr, &precedence_map))
        .sum::<i64>()
}
