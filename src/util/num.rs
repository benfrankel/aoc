pub fn gcd(a: i64, b: i64) -> i64 {
    let mut r0 = a;
    let mut r1 = b;
    while r1 > 0 {
        let tmp = r0;
        r0 = r1;
        r1 = tmp % r1;
    }
    r0
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub fn is_coprime(a: i64, b: i64) -> bool {
    gcd(a, b) == 1
}

pub fn gcd_bezout(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r0 = a;
    let mut r1 = b;
    let mut x0 = 1;
    let mut x1 = 0;
    let mut y0 = 0;
    let mut y1 = 1;
    while r1 > 0 {
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

pub fn mod_inv(a: i64, m: i64) -> i64 {
    let mut r0 = a;
    let mut r1 = m;
    let mut x0 = 1;
    let mut x1 = 0;
    while r1 > 0 {
        let q = r0 / r1;
        
        let tmp = r0;
        r0 = r1;
        r1 = tmp - q * r1;
        
        let tmp = x0;
        x0 = x1;
        x1 = tmp - q * x1;
    }
    if x0 < 0 {
        x0 + m
    } else {
        x0
    }
}

pub fn mod_inv_to(a: i64, b: i64, m: i64) -> i64 {
    (mod_inv(a, m) * b.rem_euclid(m)) % m
}

pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> (i64, i64) {
    let mut residue = 0;
    let mut modulus = 1;
    for (&next_residue, &next_modulus) in residues.iter().zip(modulii) {
        residue += modulus * mod_inv_to(
            modulus,
            next_residue - residue,
            next_modulus,
        );
        modulus *= next_modulus;
    }

    (residue, modulus)
}
