use std::collections::HashMap;

use maplit::hashmap;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

pub fn solvers(prefix: String) -> HashMap<String, crate::Solver> {
    let mut solvers = hashmap!{};

    crate::push_solver!(solvers, prefix, day01);
    crate::push_solver!(solvers, prefix, day02);
    crate::push_solver!(solvers, prefix, day03);
    crate::push_solver!(solvers, prefix, day04);
    crate::push_solver!(solvers, prefix, day05);
    crate::push_solver!(solvers, prefix, day06);
    crate::push_solver!(solvers, prefix, day07);
    crate::push_solver!(solvers, prefix, day08);
    crate::push_solver!(solvers, prefix, day09);
    crate::push_solver!(solvers, prefix, day10);
    crate::push_solver!(solvers, prefix, day11);
    
    solvers
}
