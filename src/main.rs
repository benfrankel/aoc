use itertools::Itertools;

fn main() {
    let path = std::env::args().skip(1).join("/");
    let input = "input/".to_string() + &path;
    let input = std::fs::read_to_string(&input).unwrap_or_else(|_| {
        eprintln!("Input file missing: {}", input);
        std::process::exit(1);
    });
    let solver = &solve::solvers()[&path];
    solver(&input);
}
