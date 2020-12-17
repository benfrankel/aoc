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

fn wrapping_dec(len: usize, x: usize) -> usize {
    if x == 0 {
        len - 1
    } else {
        x - 1
    }
}

fn wrapping_inc(len: usize, x: usize) -> usize {
    if x == len - 1 {
        0
    } else {
        x + 1
    }
}

fn wrap(rows: usize, cols: usize, i: isize, j: isize) -> (usize, usize) {
    (
        i.rem_euclid(rows as isize) as usize,
        j.rem_euclid(cols as isize) as usize,
    )
}

pub fn adj<'a>(
    i: isize,
    j: isize,
    steps: impl IntoIterator<Item = &'a (isize, isize)> + 'a,
) -> impl Iterator<Item = (isize, isize)> + 'a {
    steps.into_iter().map(move |(di, dj)| (i + di, j + dj))
}

pub fn bounded_adj<'a>(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
    steps: impl IntoIterator<Item = &'a (isize, isize)> + 'a,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    adj(i as isize, j as isize, steps)
        .filter(move |&(i, j)| in_bounds(rows, cols, i, j))
        .map(|(i, j)| (i as usize, j as usize))
}

pub fn wrapping_adj<'a>(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
    steps: impl IntoIterator<Item = &'a (isize, isize)> + 'a,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    adj(i as isize, j as isize, steps).map(move |(i, j)| wrap(rows, cols, i, j))
}

pub fn adj4(i: isize, j: isize) -> impl Iterator<Item = (isize, isize)> {
    adj(i, j, &STEP4)
}

pub fn bounded_adj4(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    bounded_adj(rows, cols, i, j, &STEP4)
}

pub fn wrapping_adj4(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    std::iter::empty()
        .chain(std::iter::once_with(move || (wrapping_dec(rows, i), j)))
        .chain(std::iter::once_with(move || (wrapping_inc(rows, i), j)))
        .chain(std::iter::once_with(move || (i, wrapping_dec(cols, j))))
        .chain(std::iter::once_with(move || (i, wrapping_inc(cols, j))))
}

pub fn adj8(i: isize, j: isize) -> impl Iterator<Item = (isize, isize)> {
    (i - 1..=i + 1)
        .cartesian_product(j - 1..=j + 1)
        .filter(move |&(r, c)| (r, c) != (i, j))
}

pub fn bounded_adj8(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (i.max(1) - 1..=i.min(rows - 2) + 1)
        .cartesian_product(j.max(1) - 1..=j.min(cols - 2) + 1)
        .filter(move |&(r, c)| (r, c) != (i, j))
}

pub fn wrapping_adj8(
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    std::iter::empty()
        .chain(std::iter::once_with(move || wrapping_dec(rows, i)))
        .chain(std::iter::once_with(move || i))
        .chain(std::iter::once_with(move || wrapping_inc(rows, i)))
        .cartesian_product(
            std::iter::empty()
                .chain(std::iter::once_with(move || wrapping_dec(cols, j)))
                .chain(std::iter::once_with(move || j))
                .chain(std::iter::once_with(move || wrapping_inc(cols, j))),
        )
        .filter(move |&(r, c)| (r, c) != (i, j))
        .map(|(r, c)| (r, c))
}
