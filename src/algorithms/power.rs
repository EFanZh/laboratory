pub fn power<T>(mut base: T, mut exponent: u32, init: T, mut mul: impl FnMut(&T, &T) -> T) -> T
where
    T: Clone,
{
    let mut result = init;

    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul(&result, &base);
        }

        base = mul(&base, &base);
        exponent >>= 1;
    }

    result
}
