use std::mem;

pub fn partition<T>(values: &mut [T], mut f: impl FnMut(&T) -> bool) -> usize {
    let mut result = 0;
    let mut iter = values.iter_mut();

    'outer: while let Some(left) = iter.next() {
        if !f(left) {
            loop {
                if let Some(right) = iter.next_back() {
                    if f(right) {
                        mem::swap(left, right);

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        result += 1;
    }

    result
}
