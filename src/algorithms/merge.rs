pub fn merge_1<I>(mut iter_1: I, mut iter_2: I, mut f: impl FnMut(I::Item))
where
    I: Iterator,
    I::Item: Ord,
{
    if let Some(mut left) = iter_1.next() {
        'outer: loop {
            if let Some(right) = iter_2.next() {
                while left <= right {
                    f(left);

                    if let Some(next_left) = iter_1.next() {
                        left = next_left;
                    } else {
                        left = right;

                        break 'outer;
                    }
                }

                f(right);
            } else {
                iter_2 = iter_1;

                break;
            }
        }

        f(left);
    }

    iter_2.for_each(f);
}

pub fn merge_2<I1, I2>(mut iter_1: I1, mut iter_2: I1, mut f: impl FnMut(I1::Item))
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I1::Item: Ord,
{
    if let Some(mut left) = iter_1.next() {
        'outer: loop {
            if let Some(right) = iter_2.next() {
                while left <= right {
                    f(left);

                    if let Some(next_left) = iter_1.next() {
                        left = next_left;
                    } else {
                        f(right);

                        break 'outer;
                    }
                }

                f(right);
            } else {
                f(left);

                iter_1.for_each(f);

                return;
            }
        }
    }

    iter_2.for_each(f);
}
