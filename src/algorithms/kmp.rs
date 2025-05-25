pub fn compute_prefix_function<T>(pattern: &[T]) -> Box<[usize]>
where
    T: Eq,
{
    let mut result = vec![0; pattern.len()].into_boxed_slice();

    if let Some(rest) = pattern.get(1..) {
        let mut matched = 0;
        let mut i = 0;

        for c in rest {
            i += 1;

            loop {
                if pattern[matched] == *c {
                    matched += 1;
                    result[i] = matched;
                } else if let Some(&new_matched) = result.get(matched.wrapping_sub(1)) {
                    matched = new_matched;

                    continue;
                }

                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compute_prefix_function() {
        let test_cases = [
            ("", &[] as &[usize]),
            ("a", &[0]),
            ("aa", &[0, 1]),
            ("ab", &[0, 0]),
            ("ababaca", &[0, 0, 1, 2, 3, 0, 1]),
        ];

        for (pattern, expected) in test_cases {
            assert_eq!(super::compute_prefix_function(pattern.as_bytes()).as_ref(), expected);
        }
    }
}
