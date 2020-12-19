use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub enum Token<Op> {
    Num(i64),
    Op(Op),
    LParen,
    RParen,
}

pub struct OpInfo {
    pub precedence: i64,
    pub eval: fn(i64, i64) -> i64,
}

pub fn eval<Op>(expr: &[Token<Op>], op_info: &HashMap<Op, OpInfo>) -> i64
where
    Op: Eq + Hash,
{
    fn acc<Op>(
        num_stack: &mut Vec<i64>,
        op_stack: &mut Vec<&Op>,
        op_info: &HashMap<Op, OpInfo>,
        precedence: i64,
    ) where
        Op: Eq + Hash,
    {
        while num_stack.len() >= 2
            && !op_stack.is_empty()
            && op_info[&op_stack.last().unwrap()].precedence >= precedence
        {
            let op = op_stack.pop().unwrap();
            let b = num_stack.pop().unwrap();
            let a = num_stack.pop().unwrap();
            num_stack.push((op_info[&op].eval)(a, b));
        }
    };

    let mut num_stack = vec![];
    let mut op_stack = vec![];
    for token in std::iter::once(&Token::LParen)
        .chain(expr)
        .chain(std::iter::once(&Token::RParen))
    {
        match token {
            Token::Num(x) => num_stack.push(*x),
            Token::Op(op) => {
                acc(
                    &mut num_stack,
                    op_stack.last_mut().unwrap(),
                    &op_info,
                    op_info[&op].precedence,
                );
                op_stack.last_mut().unwrap().push(op);
            }
            Token::LParen => {
                op_stack.push(vec![]);
            }
            Token::RParen => {
                acc(&mut num_stack, op_stack.last_mut().unwrap(), &op_info, -1);
                op_stack.pop();
            }
        }
    }

    num_stack[0]
}
