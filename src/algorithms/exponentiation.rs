pub fn exponentiation(base: u32, exponent: u32) -> u32 {
    if exponent == 0 {
        1
    } else {
        exponentiation_non_zero(base, exponent)
    }
}

pub fn exponentiation_non_zero(mut base: u32, mut exponent: u32) -> u32 {
    let mut result = 1;

    loop {
        if exponent & 1 != 0 {
            result *= base;

            if exponent == 1 {
                break;
            }
        }

        exponent >>= 1;
        base *= base;
    }

    result
}
