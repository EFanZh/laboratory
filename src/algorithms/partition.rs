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
    let mut iter_start = 0;
    let mut iter_end = values.len();

    'outer: loop {
        if iter_start < iter_end {
            let left_ordering = f(&values[iter_start]);

            match left_ordering {
                Ordering::Less => {}
                Ordering::Equal => break,
                Ordering::Greater => loop {
                    iter_end -= 1;

                    if iter_start < iter_end {
                        let right_ordering = f(&values[iter_end]);

                        if right_ordering != Ordering::Greater {
                            values.swap(iter_start, iter_end);

                            if right_ordering == Ordering::Less {
                                break;
                            }

                            break 'outer;
                        }
                    } else {
                        return (iter_start, iter_start);
                    }
                },
            }

            iter_start += 1;
        } else {
            return (iter_start, iter_start);
        }
    }

    let mut less_count = iter_start;

    'outer: loop {
        iter_start += 1;

        if iter_start < iter_end {
            let left_ordering = f(&values[iter_start]);

            match left_ordering {
                Ordering::Less => {
                    values.swap(less_count, iter_start);
                    less_count += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => loop {
                    iter_end -= 1;

                    if iter_start < iter_end {
                        let right_ordering = f(&values[iter_end]);

                        if right_ordering != Ordering::Greater {
                            values.swap(iter_start, iter_end);

                            if right_ordering == Ordering::Less {
                                values.swap(less_count, iter_start);
                                less_count += 1;
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

    (less_count, iter_start)
}
