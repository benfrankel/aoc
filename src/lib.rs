#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

pub mod aoc;
pub mod prelude;
pub mod util;

pub type Solver = Box<dyn Fn(&str)>;

#[macro_export]
macro_rules! push_solver {
    ($solvers:ident, $prefix:ident, $day:ident) => {
        $solvers.insert(
            $prefix.clone() + stringify!($day),
            Box::new(|input: &str| {
                let parsed = $day::parse(input);
                println!("Part 1: {}", $day::part1(&parsed));
                println!("Part 2: {}", $day::part2(&parsed));
            }) as crate::Solver,
        );
    }
}

pub fn solvers() -> HashMap<String, crate::Solver> {
    let mut solvers = HashMap::new();
    solvers.extend(aoc::solvers("aoc/".to_string()));
    solvers
}
