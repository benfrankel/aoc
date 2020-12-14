use crate::prelude::*;

pub fn parse(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect()
}

pub fn part1(input: &[(char, i64)]) -> i64 {
    let mut deltas = hashmap![
        'N' => vec2(0, 1),
        'E' => vec2(1, 0),
        'S' => vec2(0, -1),
        'W' => vec2(-1, 0),
        'F' => vec2(1, 0),
    ];
    let mut pos = vec2(0, 0);
    for action in input {
        match action.0 {
            'N' | 'E' | 'S' | 'W' | 'F' => {
                pos += action.1 * deltas[&action.0];
            }
            'R' => {
                let rotation = action.1 as usize / 90;
                for _ in 0..rotation {
                    deltas.entry('F').and_modify(|d| *d = vec2(d.y, -d.x));
                }
            }
            'L' => {
                let rotation = (360 - action.1) as usize / 90;
                for _ in 0..rotation {
                    deltas.entry('F').and_modify(|d| *d = vec2(d.y, -d.x));
                }
            }
            _ => panic!("Unknown action: {}", action.0),
        }
    }

    pos.abs().sum()
}

pub fn part2(input: &[(char, i64)]) -> i64 {
    let mut deltas = hashmap![
        'N' => vec2(0, 1),
        'E' => vec2(1, 0),
        'S' => vec2(0, -1),
        'W' => vec2(-1, 0),
        'F' => vec2(10, 1),
    ];
    let mut pos = vec2(0, 0);
    for action in input {
        match action.0 {
            'N' | 'E' | 'S' | 'W' => {
                let step = deltas[&action.0];
                deltas.entry('F').and_modify(|d| *d += action.1 * step);
            }
            'F' => {
                pos += action.1 * deltas[&'F'];
            }
            'R' => {
                let rotation = action.1 as usize / 90;
                for _ in 0..rotation {
                    deltas.entry('F').and_modify(|d| *d = vec2(d.y, -d.x));
                }
            }
            'L' => {
                let rotation = (360 - action.1) as usize / 90;
                for _ in 0..rotation {
                    deltas.entry('F').and_modify(|d| *d = vec2(d.y, -d.x));
                }
            }
            _ => panic!("Unknown action: {}", action.0),
        }
    }

    pos.abs().sum()
}
