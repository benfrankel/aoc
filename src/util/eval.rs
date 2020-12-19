use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum Token<Op> {
    Num(i64),
    Op(Op),
    LParen,
    RParen,
}

impl<Op> Token<Op> {
    pub fn is_paren(&self) -> bool {
        if let Self::LParen | Self::RParen = self {
            true
        } else {
            false
        }
    }
}

pub struct OpInfo {
    pub precedence: i64,
    pub eval: fn(i64, i64) -> i64,
}

pub fn to_postfix<Op>(
    expr: Vec<Token<Op>>,
    precedence: &HashMap<Op, i64>,
) -> Vec<Token<Op>>
where
    Op: Eq + Hash,
{
    let mut postfix = Vec::with_capacity(expr.len());
    let mut op_stack = Vec::with_capacity(expr.len());
    for token in std::iter::once(Token::LParen)
        .chain(expr)
        .chain(std::iter::once(Token::RParen))
    {
        match token {
            Token::Num(_) => postfix.push(token),
            Token::Op(ref op) => {
                let i = op_stack
                    .iter()
                    .rposition(|token| {
                        if let Token::Op(prev_op) = token {
                            precedence[&prev_op] < precedence[op]
                        } else {
                            true
                        }
                    })
                    .unwrap();
                postfix.extend(op_stack.drain(i + 1..).rev());
                op_stack.push(token);
            }
            Token::LParen => {
                op_stack.push(token);
            }
            Token::RParen => {
                let i = op_stack
                    .iter()
                    .rposition(|token| token.is_paren())
                    .unwrap();
                postfix.extend(op_stack.drain(i + 1..).rev());
                op_stack.pop();
            }
        }
    }

    postfix
}

pub fn eval_postfix<Op>(
    expr: &[Token<Op>],
    ops: &HashMap<Op, Box<dyn Fn(i64, i64) -> i64>>,
) -> i64
where
    Op: Eq + Hash + Clone,
{
    let mut num_stack = Vec::with_capacity(expr.len());
    for token in expr.iter() {
        match token {
            Token::Num(x) => num_stack.push(x.clone()),
            Token::Op(op) => {
                let b = num_stack.pop().unwrap();
                let a = num_stack.last_mut().unwrap();
                *a = (ops[&op])(b, *a);
            }
            _ => panic!("Postfix expression shouldn't include parentheses."),
        }
    }
    debug_assert_eq!(num_stack.len(), 1);

    num_stack[0]
}

pub fn eval_infix<Op>(
    expr: &[Token<Op>],
    precedence: &HashMap<Op, i64>,
    ops: &HashMap<Op, Box<dyn Fn(i64, i64) -> i64>>,
) -> i64
where
    Op: Eq + Hash,
{
    let mut num_stack = Vec::with_capacity(expr.len());
    let mut op_stack: Vec<&_> = Vec::with_capacity(expr.len());
    for token in std::iter::once(&Token::LParen)
        .chain(expr)
        .chain(std::iter::once(&Token::RParen))
    {
        match token {
            Token::Num(x) => num_stack.push(x.clone()),
            Token::Op(op) => {
                let i = op_stack
                    .iter()
                    .rposition(|token| {
                        if let Token::Op(prev_op) = token {
                            precedence[&prev_op] < precedence[op]
                        } else {
                            true
                        }
                    })
                    .unwrap();

                for token in op_stack.drain(i + 1..).rev() {
                    if let Token::Op(op) = token {
                        let b = num_stack.pop().unwrap();
                        let a = num_stack.last_mut().unwrap();
                        *a = (ops[&op])(b, *a);
                    }
                }

                op_stack.push(token);
            }
            Token::LParen => {
                op_stack.push(token);
            }
            Token::RParen => {
                let i = op_stack
                    .iter()
                    .rposition(|token| token.is_paren())
                    .unwrap();

                for token in op_stack.drain(i + 1..).rev() {
                    if let Token::Op(op) = token {
                        let b = num_stack.pop().unwrap();
                        let a = num_stack.last_mut().unwrap();
                        *a = (ops[&op])(b, *a);
                    }
                }

                op_stack.pop();
            }
        }
    }
    debug_assert_eq!(num_stack.len(), 1);

    num_stack[0]
}
