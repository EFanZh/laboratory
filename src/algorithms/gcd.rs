use std::num::NonZeroU32;

pub fn gcd(mut x: u32, mut y: u32) -> u32 {
    while let Some(non_zero_y) = NonZeroU32::new(y) {
        let z = x % non_zero_y;

        x = y;
        y = z;
    }

    x
}
