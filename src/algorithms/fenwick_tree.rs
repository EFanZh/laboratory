//! `tree[i - 1]` counts the numbers in range [`i & (i - 1)`, `i`):
//!
//! 0001 - 1 => 0000..0001
//! 0010 - 1 => 0000..0010
//! 0011 - 1 => 0010..0011
//! 0100 - 1 => 0000..0100
//! 0101 - 1 => 0100..0101
//! 0110 - 1 => 0100..0110
//! 0111 - 1 => 0110..0111
//! 1000 - 1 => 0000..1000
//! 1001 - 1 => 1000..1001
//! 1010 - 1 => 1000..1010
//! 1011 - 1 => 1010..1011
//! 1100 - 1 => 1000..1100
//! 1101 - 1 => 1100..1101
//! 1110 - 1 => 1100..1110
//! 1111 - 1 => 1110..1111

pub fn fenwick_tree_count_less_than(tree: &[u32], mut x: usize) -> u32 {
    let mut result = 0;

    loop {
        let x_minus_1 = x.wrapping_sub(1);

        if let Some(&count) = tree.get(x_minus_1) {
            result += count;

            x &= x_minus_1;
        } else {
            break;
        }
    }

    result
}

pub fn fenwick_tree_add(tree: &mut [u32], mut x: usize) {
    while let Some(count) = tree.get_mut(x) {
        *count += 1;

        x |= x + 1;
    }
}

#[test]
fn feature() {
    for x in 0..16 {
        println!("{x:b} => 0b_{:b}..0b_{:b}", (x + 1) & x, x + 1);
    }
}
