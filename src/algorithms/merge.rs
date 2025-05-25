pub fn merge_1<'a, I>(mut left_iter: I, mut right_iter: I, mut buffer_iter: impl Iterator<Item = &'a mut I::Item>)
where
    I: Iterator,
    I::Item: Ord + 'a,
{
    if let Some(mut left) = left_iter.next() {
        let mut write = |value| *buffer_iter.next().unwrap() = value;

        'outer: for right in right_iter.by_ref() {
            while left <= right {
                write(left);

                if let Some(next_left) = left_iter.next() {
                    left = next_left;
                } else {
                    left = right;
                    left_iter = right_iter;

                    break 'outer;
                }
            }

            write(right);
        }

        write(left);
    } else {
        left_iter = right_iter;
    }

    buffer_iter.zip(left_iter).for_each(|(target, value)| *target = value);
}

pub fn merge_2<'a, I1, I2>(mut left_iter: I1, mut right_iter: I1, mut buffer: impl Iterator<Item = &'a mut I1::Item>)
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I1::Item: Ord + 'a,
{
    'block: {
        if let Some(mut left) = left_iter.next() {
            let mut write = |value| *buffer.next().unwrap() = value;

            for right in right_iter.by_ref() {
                while left <= right {
                    write(left);

                    if let Some(next_left) = left_iter.next() {
                        left = next_left;
                    } else {
                        write(right);

                        break 'block;
                    }
                }

                write(right);
            }

            write(left);

            buffer.zip(left_iter).for_each(|(target, value)| *target = value);

            return;
        }
    }

    buffer.zip(right_iter).for_each(|(target, value)| *target = value);
}
