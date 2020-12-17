use itertools::Itertools;

pub const STEP4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const STEP8: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn in_bounds(rows: usize, cols: usize, i: isize, j: isize) -> bool {
    0 <= i && (i as usize) < rows && 0 <= j && (j as usize) < cols
}

pub fn adj<'a>(
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
    steps: impl IntoIterator<Item = &'a (isize, isize)> + 'a,
) -> impl Iterator<Item = (isize, isize)> + 'a {
    steps
        .into_iter()
        .map(move |(di, dj)| (i + di, j + dj))
        .filter(move |(i, j)| in_bounds(rows, cols, *i, *j))
}

pub fn adj4(
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
) -> impl Iterator<Item = (isize, isize)> {
    adj(rows, cols, i, j, &STEP4)
}

pub fn adj8(
    rows: usize,
    cols: usize,
    i: isize,
    j: isize,
) -> impl Iterator<Item = (isize, isize)> {
    ((i - 1).max(0)..=(i + 1).min(rows as isize - 1))
        .cartesian_product((j - 1).max(0)..=(j + 1).min(cols as isize - 1))
        .filter(move |(r, c)| (*r, *c) != (i, j))
}
