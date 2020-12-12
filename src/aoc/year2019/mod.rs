use std::collections::HashMap;

use maplit::hashmap;

mod day01;
mod day02;

pub fn solvers(prefix: String) -> HashMap<String, crate::Solver> {
    let mut solvers = hashmap!{};

    crate::push_solver!(solvers, prefix, day01);
    crate::push_solver!(solvers, prefix, day02);
    
    solvers
}
