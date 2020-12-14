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

pub fn bit_positions(mut n: u64) -> Vec<usize> {
    let mut bits = vec![];
    let mut shifted = 0;
    while n > 0 {
        let bit = n.trailing_zeros();
        bits.push((shifted + bit) as usize);
        
        n >>= bit + 1;
        shifted += bit + 1;
    }

    bits
}
