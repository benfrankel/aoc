use std::collections::HashMap;

fn main() {
    solve(&solve::solvers(), "aoc/year2019/day02".to_string());
}

fn solve(
    solvers: &HashMap<String, solve::Solver>,
    key: String,
) {
    let input = "input/".to_string() + &key;
    let input = std::fs::read_to_string(input).unwrap();
    solvers[&key](&input);
}
