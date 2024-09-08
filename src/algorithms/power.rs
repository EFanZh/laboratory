pub fn power<T>(mut base: T, mut exponent: u32, init: T, mut mul: impl FnMut(&T, &T) -> T) -> T
where
    T: Clone,
{
    let mut result = init;

    if exponent != 0 {
        loop {
            if exponent & 1 != 0 {
                result = mul(&result, &base);
            }

            exponent >>= 1;

            if exponent == 0 {
                break;
            }

            base = mul(&base, &base);
        }
    }

    result
}
