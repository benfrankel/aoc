use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub};

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + From<bool> + PartialOrd + Rem<Output = T>,
{
    let zero = false.into();

    let mut r0 = a;
    let mut r1 = b;
    while r1 > zero {
        let tmp = r0;
        r0 = r1;
        r1 = tmp % r1;
    }
    r0
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + From<bool>
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>,
{
    a / gcd(a, b) * b
}

pub fn is_coprime<T>(a: T, b: T) -> bool
where
    T: Copy + From<bool> + PartialOrd + Rem<Output = T>,
{
    let one = true.into();
    gcd(a, b) == one
}

pub fn gcd_bezout<T>(a: T, b: T) -> (T, T, T)
where
    T: Copy
        + From<i8>
        + PartialOrd
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    let zero = 0.into();
    let one = 1.into();

    let mut r0 = a;
    let mut r1 = b;
    let mut x0 = one;
    let mut x1 = zero;
    let mut y0 = zero;
    let mut y1 = one;
    while r1 > zero {
        let q = r0 / r1;

        let tmp = r0;
        r0 = r1;
        r1 = tmp - q * r1;

        let tmp = x0;
        x0 = x1;
        x1 = tmp - q * x1;

        let tmp = y0;
        y0 = y1;
        y1 = tmp - q * y1;
    }
    (r0, x0, y0)
}

pub fn mod_inv<T>(a: T, m: T) -> T
where
    T: Copy
        + From<i8>
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    let zero = 0.into();
    let one = 1.into();

    let mut r0 = a;
    let mut r1 = m;
    let mut x0 = one;
    let mut x1 = zero;
    while r1 > zero {
        let q = r0 / r1;

        let tmp = r0;
        r0 = r1;
        r1 = tmp - q * r1;

        let tmp = x0;
        x0 = x1;
        x1 = tmp - q * x1;
    }
    if x0 < zero {
        x0 + m
    } else {
        x0
    }
}

pub fn mod_inv_to<T>(a: T, b: T, m: T) -> T
where
    T: Copy
        + From<i8>
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>,
{
    let zero = 0.into();

    (mod_inv(a, m) * {
        let r = b % m;
        if r < zero {
            if m < zero {
                r - m
            } else {
                r + m
            }
        } else {
            r
        }
    }) % m
}

pub fn chinese_remainder<T>(residues: &[T], modulii: &[T]) -> (T, T)
where
    T: Copy
        + From<i8>
        + PartialOrd
        + Add<Output = T>
        + AddAssign<T>
        + Sub<Output = T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + Rem<Output = T>,
{
    let mut residue = 0.into();
    let mut modulus = 1.into();
    for (&next_residue, &next_modulus) in residues.iter().zip(modulii) {
        residue +=
            modulus * mod_inv_to(modulus, next_residue - residue, next_modulus);
        modulus *= next_modulus;
    }

    (residue, modulus)
}
