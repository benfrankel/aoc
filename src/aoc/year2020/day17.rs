use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Active,
    Inactive,
}

pub fn parse(input: &str) -> Vec<Vec<State>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    _ => panic!("Unknown state: {}", c),
                })
                .collect()
        })
        .collect()
}

fn adj8_3d(
    lyrs: usize,
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
    k: isize,
) -> Vec<(isize, isize, isize)> {
    ((i - 1).max(0)..=(i + 1).min(lyrs as isize - 1))
        .cartesian_product((j - 1).max(0)..=(j + 1).min(rows as isize - 1))
        .cartesian_product((k - 1).max(0)..=(k + 1).min(cols as isize - 1))
        .map(|((l, r), c)| (l, r, c))
        .filter(|(l, r, c)| (*l, *r, *c) != (i, j, k))
        .collect()
}

fn in_bounds_3d(
    lyrs: usize,
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
    k: isize,
) -> bool {
    0 <= i
        && (i as usize) < lyrs
        && 0 <= j
        && (j as usize) < rows
        && 0 <= k
        && (k as usize) < cols
}

pub fn part1(input: &[Vec<State>]) -> i64 {
    let mut grid = vec![input.to_vec()];

    for _ in 0..6 {
        let lyrs = grid.len();
        let rows = grid[0].len();
        let cols = grid[0][0].len();

        let new_lyrs = lyrs + 2;
        let new_rows = rows + 2;
        let new_cols = cols + 2;
        let mut new_grid = vec![];

        for new_i in 0..new_lyrs {
            let i = new_i as isize - 1;
            new_grid.push(vec![]);
            for new_j in 0..new_rows {
                let j = new_j as isize - 1;
                new_grid[new_i].push(vec![]);
                for new_k in 0..new_cols {
                    let k = new_k as isize - 1;

                    let state = if in_bounds_3d(lyrs, rows, cols, i, j, k) {
                        grid[i as usize][j as usize][k as usize]
                    } else {
                        State::Inactive
                    };
                    let num_active = adj8_3d(lyrs, rows, cols, i, j, k)
                        .iter()
                        .filter(|(i, j, k)| {
                            grid[*i as usize][*j as usize][*k as usize]
                                == State::Active
                        })
                        .count();

                    new_grid[new_i][new_j].push(match (state, num_active) {
                        (State::Active, 2)
                        | (State::Active, 3)
                        | (State::Inactive, 3) => State::Active,
                        _ => State::Inactive,
                    });
                }
            }
        }

        std::mem::swap(&mut grid, &mut new_grid);
    }

    grid.iter()
        .map(|lyr| {
            lyr.iter()
                .map(|row| {
                    row.iter().filter(|state| **state == State::Active).count()
                        as i64
                })
                .sum::<i64>()
        })
        .sum()
}

fn adj8_4d(
    spcs: usize,
    lyrs: usize,
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
    k: isize,
    u: isize,
) -> Vec<(isize, isize, isize, isize)> {
    ((i - 1).max(0)..=(i + 1).min(spcs as isize - 1))
        .cartesian_product((j - 1).max(0)..=(j + 1).min(lyrs as isize - 1))
        .cartesian_product((k - 1).max(0)..=(k + 1).min(rows as isize - 1))
        .cartesian_product((u - 1).max(0)..=(u + 1).min(cols as isize - 1))
        .map(|(((s, l), r), c)| (s, l, r, c))
        .filter(|(s, l, r, c)| (*s, *l, *r, *c) != (i, j, k, u))
        .collect()
}

fn in_bounds_4d(
    spcs: usize,
    lyrs: usize,
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
    k: isize,
    u: isize,
) -> bool {
    0 <= i
        && (i as usize) < spcs
        && 0 <= j
        && (j as usize) < lyrs
        && 0 <= k
        && (k as usize) < rows
        && 0 <= u
        && (u as usize) < cols
}

pub fn part2(input: &[Vec<State>]) -> i64 {
    let mut grid = vec![vec![input.to_vec()]];

    for _ in 0..6 {
        let spcs = grid.len();
        let lyrs = grid[0].len();
        let rows = grid[0][0].len();
        let cols = grid[0][0][0].len();

        let new_spcs = spcs + 2;
        let new_lyrs = lyrs + 2;
        let new_rows = rows + 2;
        let new_cols = cols + 2;
        let mut new_grid = vec![];

        for new_i in 0..new_spcs {
            let i = new_i as isize - 1;
            new_grid.push(vec![]);
            for new_j in 0..new_lyrs {
                let j = new_j as isize - 1;
                new_grid[new_i].push(vec![]);
                for new_k in 0..new_rows {
                    let k = new_k as isize - 1;
                    new_grid[new_i][new_j].push(vec![]);
                    for new_u in 0..new_cols {
                        let u = new_u as isize - 1;

                        let state =
                            if in_bounds_4d(spcs, lyrs, rows, cols, i, j, k, u)
                            {
                                grid[i as usize][j as usize][k as usize]
                                    [u as usize]
                            } else {
                                State::Inactive
                            };
                        let num_active =
                            adj8_4d(spcs, lyrs, rows, cols, i, j, k, u)
                                .iter()
                                .filter(|(i, j, k, u)| {
                                    grid[*i as usize][*j as usize][*k as usize]
                                        [*u as usize]
                                        == State::Active
                                })
                                .count();

                        new_grid[new_i][new_j][new_k].push(
                            match (state, num_active) {
                                (State::Active, 2)
                                | (State::Active, 3)
                                | (State::Inactive, 3) => State::Active,
                                _ => State::Inactive,
                            },
                        );
                    }
                }
            }
        }

        std::mem::swap(&mut grid, &mut new_grid);
    }

    grid.iter()
        .map(|spc| {
            spc.iter()
                .map(|lyr| {
                    lyr.iter()
                        .map(|row| {
                            row.iter()
                                .filter(|state| **state == State::Active)
                                .count() as i64
                        })
                        .sum::<i64>()
                })
                .sum::<i64>()
        })
        .sum()
}
