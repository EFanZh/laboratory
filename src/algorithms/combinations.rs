pub fn combinations(n: u8, k: u8, base: u16, f: &mut impl FnMut(u16)) {
    if k == 0 {
        f(base);
    } else {
        let next_k = k - 1;

        for i in next_k..n {
            combinations(i, next_k, base | (1 << i), f);
        }
    }
}
