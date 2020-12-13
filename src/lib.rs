#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub mod aoc;
pub mod prelude;
pub mod util;

pub type Solver = Box<dyn Fn(&str)>;

fn time<T, F>(
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

fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{}s", d.as_secs())
    } else if d.as_millis() > 0 {
        format!("{}ms", d.as_millis())
    } else if d.as_micros() > 0 {
        format!("{}μs", d.as_micros())
    } else {
        format!("{}ns", d.as_nanos())
    }
}

fn summarize_time(elapsed: &[Duration]) {
    println!(
        "    Min duration: {}",
        format_duration(*elapsed.iter().min().unwrap()),
    );
    println!(
        "    Avg duration: {}",
        format_duration(elapsed.iter().sum::<Duration>() / elapsed.len() as u32),
    );
    println!(
        "    Max duration: {}",
        format_duration(*elapsed.iter().max().unwrap()),
    );
}

#[macro_export]
macro_rules! solvers {
    ($($day:ident,)*) => {
        $(mod $day;)*
        
        pub fn solvers(prefix: String) -> std::collections::HashMap<String, crate::Solver> {
            let mut solvers = maplit::hashmap!{};

            $(
                solvers.insert(
                    prefix.clone() + stringify!($day),
                    Box::new(|input: &str| {
                        let (elapsed, parsed) = crate::time(
                            1000,
                            std::time::Duration::from_millis(100),
                            || $day::parse(input),
                        );
                        println!("Parsing:");
                        crate::summarize_time(&elapsed);
                        
                        let (elapsed, output) = crate::time(
                            1000,
                            std::time::Duration::from_millis(100),
                            || $day::part1(&parsed),
                        );
                        println!("Part 1: {}", output);
                        crate::summarize_time(&elapsed);
                        
                        let (elapsed, output) = crate::time(
                            1000,
                            std::time::Duration::from_millis(100),
                            || $day::part2(&parsed),
                        );
                        println!("Part 2: {}", output);
                        crate::summarize_time(&elapsed);
                    }) as crate::Solver,
                );
            )*
            
            solvers
        }
    }
}

pub fn solvers() -> HashMap<String, crate::Solver> {
    let mut solvers = HashMap::new();
    solvers.extend(aoc::solvers("aoc/".to_string()));
    solvers
}
