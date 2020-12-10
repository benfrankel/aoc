#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

pub mod aoc;
pub mod prelude;
pub mod util;

pub type Solver = Box<dyn Fn(&str)>;

pub fn solvers() -> HashMap<String, crate::Solver> {
    let mut solvers = HashMap::new();
    solvers.extend(aoc::solvers("aoc/".to_string()));
    solvers
}
