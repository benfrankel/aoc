use std::cmp::Ordering;

pub fn binary_to_i64<T, I>(binary: I, one: T) -> i64
where
    T: PartialEq,
    I: IntoIterator<Item = T>,
{
    let mut n = 0;
    for item in binary.into_iter() {
        n <<= 1;
        if item == one {
            n |= 1;
        }
    }
    n
}

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
