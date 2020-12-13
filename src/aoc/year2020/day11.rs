use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spot {
    Floor,
    Occupied,
    Unoccupied,
}

pub fn parse(input: &str) -> Vec<Vec<Spot>> {
    input
        .lines()
        .map(|line| line
             .chars()
             .map(|c| match c {
                 '.' => Spot::Floor,
                 '#' => Spot::Occupied,
                 'L' => Spot::Unoccupied,
                 x => panic!("Unrecognized spot: {}", x),
             })
             .collect::<Vec<_>>()
        )
        .collect()
}

fn in_bounds(spots: &[Vec<Spot>], pos: (i64, i64)) -> bool {
    let (i, j) = pos;
    0 <= i && i < spots.len() as i64 && 0 <= j && j < spots[0].len() as i64
}

fn adjacent1(spots: &[Vec<Spot>], pos: (i64, i64)) -> Vec<(i64, i64)> {
    let (i, j) = pos;
    (i - 1..=i + 1)
        .cartesian_product(j - 1..=j + 1)
        .filter(|pos| in_bounds(spots, *pos))
        .filter(|(a, b)| (*a, *b) != (i, j))
        .collect()
}

pub fn part1(input: &[Vec<Spot>]) -> i64 {
    let mut spots = input.to_vec();
    let mut new_spots = spots.clone();
    loop {
        let mut changes = 0;
        for i in 0..spots.len() as i64 {
            for j in 0..spots[0].len() as i64 {
                new_spots[i as usize][j as usize] = match spots[i as usize][j as usize] {
                    Spot::Unoccupied => if adjacent1(&spots, (i, j))
                        .iter()
                        .any(|(a, b)| spots[*a as usize][*b as usize] == Spot::Occupied) {
                            Spot::Unoccupied
                        } else {
                            Spot::Occupied
                        },
                    Spot::Occupied => if adjacent1(&spots, (i, j))
                        .iter()
                        .filter(|(a, b)| spots[*a as usize][*b as usize] == Spot::Occupied)
                        .count() >= 4 {
                            Spot::Unoccupied
                        } else {
                            Spot::Occupied
                        },
                    Spot::Floor => Spot::Floor,
                };
                if new_spots[i as usize][j as usize] != spots[i as usize][j as usize] {
                    changes += 1;
                }
            }
        }
        if changes == 0 {
            break;
        }
        std::mem::swap(&mut spots, &mut new_spots);
    }

    new_spots
        .iter()
        .map(|row| row
             .iter()
             .filter(|spot| **spot == Spot::Occupied)
             .count() as i64
        )
        .sum()
}

fn adjacent2(spots: &[Vec<Spot>], pos: (i64, i64)) -> Vec<(i64, i64)> {
    let (i, j) = pos;
    vec![
        (0..i).rev()
            .map(|a| (a, j))
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (i + 1..spots.len() as i64)
            .map(|a| (a, j))
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (0..j).rev()
            .map(|b| (i, b))
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (j + 1..spots[0].len() as i64)
            .map(|b| (i, b))
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (0..i).rev()
            .zip((0..j).rev())
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (0..i).rev()
            .zip(j + 1..spots[0].len() as i64)
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (i + 1..spots.len() as i64)
            .zip((0..j).rev())
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
        (i + 1..spots.len() as i64)
            .zip(j + 1..spots[0].len() as i64)
            .find(|(a, b)| spots[*a as usize][*b as usize] != Spot::Floor),
    ]
        .iter()
        .filter_map(|x| *x)
        .collect()
}

pub fn part2(input: &[Vec<Spot>]) -> i64 {
    let mut spots = input.to_vec();
    let mut new_spots = spots.clone();
    loop {
        let mut changes = 0;
        for i in 0..spots.len() as i64 {
            for j in 0..spots[0].len() as i64 {
                new_spots[i as usize][j as usize] = match spots[i as usize][j as usize] {
                    Spot::Unoccupied => if adjacent2(&spots, (i, j))
                        .iter()
                        .any(|(a, b)| spots[*a as usize][*b as usize] == Spot::Occupied) {
                            Spot::Unoccupied
                        } else {
                            Spot::Occupied
                        },
                    Spot::Occupied => if adjacent2(&spots, (i, j))
                        .iter()
                        .filter(|(a, b)| spots[*a as usize][*b as usize] == Spot::Occupied)
                        .count() >= 5 {
                            Spot::Unoccupied
                        } else {
                            Spot::Occupied
                        },
                    Spot::Floor => Spot::Floor,
                };
                if new_spots[i as usize][j as usize] != spots[i as usize][j as usize] {
                    changes += 1;
                }
            }
        }
        if changes == 0 {
            break;
        }
        std::mem::swap(&mut spots, &mut new_spots);
    }

    new_spots
        .iter()
        .map(|row| row
             .iter()
             .filter(|spot| **spot == Spot::Occupied)
             .count() as i64
        )
        .sum()
}
