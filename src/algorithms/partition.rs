use std::cmp::Ordering;
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

pub fn three_way_partition<T>(values: &mut [T], mut f: impl FnMut(&T) -> Ordering) -> (usize, usize) {
    // +------+-------+-----------+---------+
    // | Less | Equal | Unchecked | Greater |
    // +------+-------+-----------+---------+

    let mut unchecked_start = 0;
    let mut greater_start = values.len();

    'outer: loop {
        if unchecked_start < greater_start {
            let left_ordering = f(&values[unchecked_start]);

            match left_ordering {
                Ordering::Less => {}
                Ordering::Equal => break,
                Ordering::Greater => loop {
                    greater_start -= 1;

                    if unchecked_start < greater_start {
                        let right_ordering = f(&values[greater_start]);

                        if right_ordering != Ordering::Greater {
                            values.swap(unchecked_start, greater_start);

                            if right_ordering == Ordering::Less {
                                break;
                            }

                            break 'outer;
                        }
                    } else {
                        return (unchecked_start, unchecked_start);
                    }
                },
            }

            unchecked_start += 1;
        } else {
            return (unchecked_start, unchecked_start);
        }
    }

    let mut equal_start = unchecked_start;

    'outer: loop {
        unchecked_start += 1;

        if unchecked_start < greater_start {
            let left_ordering = f(&values[unchecked_start]);

            match left_ordering {
                Ordering::Less => {
                    values.swap(equal_start, unchecked_start);
                    equal_start += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => loop {
                    greater_start -= 1;

                    if unchecked_start < greater_start {
                        let right_ordering = f(&values[greater_start]);

                        if right_ordering != Ordering::Greater {
                            values.swap(unchecked_start, greater_start);

                            if right_ordering == Ordering::Less {
                                values.swap(equal_start, unchecked_start);
                                equal_start += 1;
                            }

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                },
            }
        } else {
            break 'outer;
        }
    }

    (equal_start, unchecked_start)
}

pub fn three_way_partition_simple<T>(values: &mut [T], mut f: impl FnMut(&T) -> Ordering) -> (usize, usize)
where
    T: Ord,
{
    // +------+-------+-----------+---------+
    // | Less | Equal | Unchecked | Greater |
    // +------+-------+-----------+---------+

    let mut equal_start = 0;
    let mut unchecked_start = 0;
    let mut greater_start = values.len();

    while unchecked_start < greater_start {
        match f(&values[unchecked_start]) {
            Ordering::Less => {
                values.swap(equal_start, unchecked_start);

                equal_start += 1;
                unchecked_start += 1;
            }
            Ordering::Equal => unchecked_start += 1,
            Ordering::Greater => {
                greater_start -= 1;

                values.swap(unchecked_start, greater_start);
            }
        }
    }

    (equal_start, unchecked_start)
}
