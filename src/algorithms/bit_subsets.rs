pub fn bit_subsets(bits: u32, mut f: impl FnMut(u32)) {
    let mut current = bits;

    loop {
        f(current);

        if current == 0 {
            break;
        }

        current = bits & (current - 1);
    }
}
