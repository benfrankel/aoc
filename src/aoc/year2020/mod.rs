use std::collections::HashMap;

use maplit::hashmap;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

pub fn solvers(prefix: String) -> HashMap<String, crate::Solver> {
    hashmap!{
        prefix.clone() + "day01" => Box::new(day01::solve) as crate::Solver,
        prefix.clone() + "day02" => Box::new(day02::solve),
        prefix.clone() + "day03" => Box::new(day03::solve),
        prefix.clone() + "day04" => Box::new(day04::solve),
        prefix.clone() + "day05" => Box::new(day05::solve),
        prefix.clone() + "day06" => Box::new(day06::solve),
        prefix.clone() + "day07" => Box::new(day07::solve),
        prefix.clone() + "day08" => Box::new(day08::solve),
        prefix.clone() + "day09" => Box::new(day09::solve),
        prefix.clone() + "day10" => Box::new(day10::solve),
    }
}
