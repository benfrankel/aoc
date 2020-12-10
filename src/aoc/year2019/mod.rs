use std::collections::HashMap;

use maplit::hashmap;

pub mod day01;

pub fn solvers(prefix: String) -> HashMap<String, crate::Solver> {
    hashmap!{
        prefix.clone() + "day01" => Box::new(day01::solve) as crate::Solver,
    }
}
