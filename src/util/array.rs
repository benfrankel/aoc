use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub fn find_sum2<T>(a: &[T], target: T) -> Option<(usize, usize)>
where
    T: Copy + Ord + Add<Output = T>,
{
    // TODO: Uncomment when is_sorted is stabilized.
    //debug_assert!(a.is_sorted());

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

pub fn find_sum3<T>(a: &[T], target: T) -> Option<(usize, usize, usize)>
where
    T: Copy + Ord + Sub<Output = T>,
{
    // TODO: Uncomment when is_sorted is stabilized.
    //debug_assert!(a.is_sorted());

    let mut i = 0;
    let mut j = a.len() - 1;
    while i + 1 < j {
        let remaining = target - a[i] - a[j];
        match a[i + 1..j].binary_search(&remaining) {
            Ok(mid) => return Some((i, i + 1 + mid, j)),
            Err(mid) => {
                if mid == j - (i + 1) {
                    i += 1;
                } else {
                    j -= 1;
                }
            }
        }
    }

    None
}

pub fn find_diff2<T>(a: &[T], target: T) -> Option<(usize, usize)>
where
    T: Copy + Ord + Sub<Output = T>,
{
    // TODO: Uncomment when is_sorted is stabilized.
    //debug_assert!(a.is_sorted());

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

pub fn find_window_sum<T>(a: &[T], mut target: T) -> Option<(usize, usize)>
where
    T: Copy + From<bool> + Ord + AddAssign<T> + SubAssign<T>,
{
    let zero = false.into();

    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < a.len() {
        match target.cmp(&zero) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Greater => {
                target -= a[j];
                j += 1;
            }
            Ordering::Less => {
                target += a[i];
                i += 1;
            }
        }
    }

    None
}

pub fn running_sum<T>(a: &[T]) -> Vec<T>
where
    T: Copy + From<bool> + AddAssign<T>,
{
    let zero = false.into();

    a.iter()
        .scan(zero, |sum, x| {
            *sum += *x;
            Some(*sum)
        })
        .collect()
}
