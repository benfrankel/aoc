use std::collections::HashMap;

use maplit::hashmap;

pub mod day01;
pub mod day02;

pub fn solvers(prefix: String) -> HashMap<String, crate::Solver> {
    hashmap!{
        prefix.clone() + "day01" => Box::new(day01::solve) as crate::Solver,
        prefix.clone() + "day02" => Box::new(day02::solve),
    }
}
