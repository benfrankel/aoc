mod bin;
mod num;
mod vec;

pub use bin::*;
pub use num::*;
pub use vec::*;

use std::cmp::Ordering;
use std::ops::AddAssign;

pub fn find_sum2(a: &[i64], target: i64) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = a.len() - 1;
    while i < j {
        let sum = a[i] + a[j];
        match sum.cmp(&target) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }

    None
}

pub fn find_sum3(a: &[i64], target: i64) -> Option<(usize, usize, usize)> {
    let mut i = 0;
    let mut j = a.len() - 1;
    while i + 1 < j {
        let remaining = target - a[i] - a[j];
        match a[i + 1..j].binary_search(&remaining) {
            Ok(mid) => return Some((i, i + 1 + mid, j)),
            Err(mid) => if mid == j - (i + 1) {
                i += 1;
            } else {
                j -= 1;
            }
        }
    }

    None
}

pub fn find_diff2(a: &[i64], target: i64) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < a.len() {
        let diff = a[j] - a[i];
        match diff.cmp(&target) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Less => j += 1,
            Ordering::Greater => i += 1,
        }
    }

    None
}

pub fn find_window_sum(a: &[i64], mut target: i64) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < a.len() {
        match target.cmp(&0) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Greater => {
                target -= a[j];
                j += 1;
            },
            Ordering::Less => {
                target += a[i];
                i += 1;
            },
        }
    }

    None
}

pub fn running_sum<T>(a: &[T]) -> Vec<T>
where
    T: Default + Copy + AddAssign<T>,
{
    a
        .iter()
        .scan(Default::default(), |sum, x| {
            *sum += *x;
            Some(*sum)
        })
        .collect()
}

pub trait StrExt {
    fn split2<'a>(&'a self, delimiter: &str) -> (String, String);
}

impl StrExt for str {
    fn split2<'a>(&'a self, delimeter: &str) -> (String, String) {
        let mut split = self.splitn(2, delimeter);
        (
            split.next().unwrap().to_string(),
            split.next().unwrap().to_string(),
        )
    }
}
