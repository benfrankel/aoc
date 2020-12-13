#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub mod aoc;
pub mod prelude;
pub mod util;

pub type Solver = Box<dyn Fn(&str)>;

pub fn time<T, F>(
    max_iterations: usize,
    max_duration: Duration,
    f: F,
) -> (Vec<Duration>, T)
where
    F: Fn() -> T,
{
    assert!(max_iterations >= 1);
    
    let mut elapsed = vec![];
    let mut output = None;
    let loop_start = Instant::now();
    for _ in 0..max_iterations {
        let start = Instant::now();
        output = Some(f());
        let end = Instant::now();
        
        elapsed.push(end - start);
        if end - loop_start >= max_duration {
            break;
        }
    }
    
    (elapsed, output.unwrap())
}

pub fn summarize(elapsed: &[Duration]) {
    println!(
        "    Min duration: {} μs",
        elapsed.iter().min().unwrap().as_micros(),
    );
    println!(
        "    Avg duration: {} μs",
        elapsed.iter().sum::<Duration>().as_micros() as f32 / elapsed.len() as f32,
    );
    println!(
        "    Max duration: {} μs",
        elapsed.iter().max().unwrap().as_micros(),
    );
}

#[macro_export]
macro_rules! push_solver {
    ($solvers:ident, $prefix:ident, $day:ident) => {
        {
            $solvers.insert(
                $prefix.clone() + stringify!($day),
                Box::new(|input: &str| {
                    let (elapsed, parsed) = crate::time(
                        1000,
                        std::time::Duration::from_millis(100),
                        || $day::parse(input),
                    );
                    println!("Parsing:");
                    crate::summarize(&elapsed);
                    
                    let (elapsed, output) = crate::time(
                        1000,
                        std::time::Duration::from_millis(100),
                        || $day::part1(&parsed),
                    );
                    println!("Part 1: {}", output);
                    crate::summarize(&elapsed);
                    
                    let (elapsed, output) = crate::time(
                        1000,
                        std::time::Duration::from_millis(100),
                        || $day::part2(&parsed),
                    );
                    println!("Part 2: {}", output);
                    crate::summarize(&elapsed);
                }) as crate::Solver,
            );
        }
    }
}

pub fn solvers() -> HashMap<String, crate::Solver> {
    let mut solvers = HashMap::new();
    solvers.extend(aoc::solvers("aoc/".to_string()));
    solvers
}
