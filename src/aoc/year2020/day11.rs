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
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Spot::Floor,
                    '#' => Spot::Occupied,
                    'L' => Spot::Unoccupied,
                    x => panic!("Unrecognized spot: {}", x),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part1(input: &[Vec<Spot>]) -> i64 {
    let rows = input.len();
    let cols = input[0].len();

    let mut spots = input.to_vec();
    let mut new_spots = spots.clone();
    loop {
        let mut changed = false;
        for i in 0..rows {
            for j in 0..cols {
                if spots[i][j] == Spot::Floor {
                    continue;
                }

                let num_occupied = adj8(rows, cols, i as isize, j as isize)
                    .filter(|(a, b)| {
                        spots[*a as usize][*b as usize] == Spot::Occupied
                    })
                    .count();

                new_spots[i][j] = match spots[i][j] {
                    Spot::Unoccupied if num_occupied == 0 => Spot::Occupied,
                    Spot::Occupied if num_occupied >= 4 => Spot::Unoccupied,
                    x => x,
                };
                if new_spots[i][j] != spots[i][j] {
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        std::mem::swap(&mut spots, &mut new_spots);
    }

    new_spots
        .iter()
        .map(|row| {
            row.iter().filter(|spot| **spot == Spot::Occupied).count() as i64
        })
        .sum()
}

fn adjacent2(
    spots: &[Vec<Spot>],
    i: isize,
    j: isize,
) -> impl Iterator<Item = (isize, isize)> + 'static {
    std::iter::empty()
        .chain(
            (0..i)
                .rev()
                .map(|a| (a, j))
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (i + 1..spots.len() as isize)
                .map(|a| (a, j))
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (0..j)
                .rev()
                .map(|b| (i, b))
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (j + 1..spots[0].len() as isize)
                .map(|b| (i, b))
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (0..i)
                .rev()
                .zip((0..j).rev())
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (0..i)
                .rev()
                .zip(j + 1..spots[0].len() as isize)
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (i + 1..spots.len() as isize)
                .zip((0..j).rev())
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
        .chain(
            (i + 1..spots.len() as isize)
                .zip(j + 1..spots[0].len() as isize)
                .find(|&(a, b)| spots[a as usize][b as usize] != Spot::Floor),
        )
}

pub fn part2(input: &[Vec<Spot>]) -> i64 {
    let mut spots = input.to_vec();
    let mut new_spots = spots.clone();
    loop {
        let mut changed = false;
        for i in 0..spots.len() {
            for j in 0..spots[0].len() {
                if spots[i][j] == Spot::Floor {
                    continue;
                }

                let num_occupied = adjacent2(&spots, i as isize, j as isize)
                    .filter(|(a, b)| {
                        spots[*a as usize][*b as usize] == Spot::Occupied
                    })
                    .count();

                new_spots[i][j] = match spots[i][j] {
                    Spot::Unoccupied if num_occupied == 0 => Spot::Occupied,
                    Spot::Occupied if num_occupied >= 5 => Spot::Unoccupied,
                    x => x,
                };
                if new_spots[i][j] != spots[i][j] {
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        std::mem::swap(&mut spots, &mut new_spots);
    }

    new_spots
        .iter()
        .map(|row| {
            row.iter().filter(|spot| **spot == Spot::Occupied).count() as i64
        })
        .sum()
}
