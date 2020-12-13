pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> (i64, i64) {
    let mut residue = 0;
    let mut modulus = 1;
    for (next_residue, next_modulus) in residues.iter().zip(modulii.iter()) {
        while residue % next_modulus != *next_residue {
            residue += modulus;
        }
        modulus *= next_modulus;
    }

    (residue, modulus)
}
