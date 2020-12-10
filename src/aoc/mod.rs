use std::collections::HashMap;

pub mod year2019;
pub mod year2020;

pub fn solvers(prefix: String) -> HashMap<String, crate::Solver> {
    let mut solvers = HashMap::new();
    solvers.extend(year2019::solvers(prefix.clone() + "year2019/"));
    solvers.extend(year2020::solvers(prefix.clone() + "year2020/"));
    solvers
}
