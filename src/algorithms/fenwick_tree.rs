//! `tree[i]` stores the count of numbers in range `(i & (i + 1))..=i`.
//!
//! 0000 => [0000, 0000]
//! 0001 => [0000, 0001]
//! 0010 => [0010, 0010]
//! 0011 => [0000, 0011]
//! 0100 => [0100, 0100]
//! 0101 => [0100, 0101]
//! 0110 => [0110, 0110]
//! 0111 => [0000, 0111]
//! 1000 => [1000, 1000]
//! 1001 => [1000, 1001]
//! 1010 => [1010, 1010]
//! 1011 => [1000, 1011]
//! 1100 => [1100, 1100]
//! 1101 => [1100, 1101]
//! 1110 => [1110, 1110]

pub fn fenwick_tree_add(tree: &mut [u32], mut x: usize) {
    while let Some(count) = tree.get_mut(x) {
        *count += 1;

        x |= x + 1;
    }
}

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
