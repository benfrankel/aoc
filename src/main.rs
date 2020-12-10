use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let path = std::env::args().skip(1).join("/");
    solve(&solve::solvers(), path);
}

fn solve(
    solvers: &HashMap<String, solve::Solver>,
    key: String,
) {
    let input = "input/".to_string() + &key;
    let input = match std::fs::read_to_string(&input) {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Input file missing: {}", input);
            std::process::exit(1);
        },
    };
    solvers[&key](&input);
}
