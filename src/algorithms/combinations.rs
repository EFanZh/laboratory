pub fn combinations(n: u8, mut k: u8, base: u16, f: &mut impl FnMut(u16)) {
    if k == 0 {
        f(base);
    } else {
        k -= 1;

        for i in k..n {
            combinations(i, k, base | (1 << i), f);
        }
    }
}
