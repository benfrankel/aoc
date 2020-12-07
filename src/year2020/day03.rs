pub use crate::prelude::*;

#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    let mut count = 0;
    let mut col = 0;
    for line in input.lines() {
        if line.chars().nth(col).unwrap() == '#' {
            count += 1;
        }
        
        col += 3;
        let len = line.chars().count();
        if col >= len {
            col -= len;
        }
    }

    count
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
    let slope = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    
    let mut count = vec![0; 5];
    let mut col = vec![0; 5];
    for (i, line) in input.lines().enumerate() {
        for j in 0..5 {
            if i % slope[j].1 != 0 {
                continue;
            }

            if line.chars().nth(col[j]).unwrap() == '#' {
                count[j] += 1;
            }
        
            col[j] += slope[j].0;
            let len = line.chars().count();
            if col[j] >= len {
                col[j] -= len;
            }
        }
    }

    count.iter().product()
}
