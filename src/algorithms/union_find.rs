use std::cell::Cell;
use std::{mem, ptr};

fn get_node(array: &[(Cell<usize>, Cell<usize>)], value: &(Cell<usize>, Cell<usize>)) -> usize {
    ((ptr::from_ref(value) as usize - array.as_ptr() as usize)
        / mem::size_of::<(Cell<usize>, Cell<usize>)>()) as _
}

pub fn find_root<'a>(
    union_find: &'a [(Cell<usize>, Cell<usize>)],
    node_state: &'a (Cell<usize>, Cell<usize>),
) -> &'a (Cell<usize>, Cell<usize>) {
    let parent = node_state.0.get();

    union_find.get(parent).map_or(node_state, |parent_state| {
        let root = find_root(union_find, parent_state);

        node_state.0.set(get_node(union_find, root));

        root
    })
}

pub fn union(
    union_find: &[(Cell<usize>, Cell<usize>)],
    left_state: &(Cell<usize>, Cell<usize>),
    right_state: &(Cell<usize>, Cell<usize>),
) {
    let left_root = find_root(union_find, left_state);
    let right_root = find_root(union_find, right_state);

    if !ptr::eq(left_root, right_root) {
        let left_rank = left_root.1.get();
        let right_rank = right_root.1.get();

        if left_rank < right_rank {
            left_root.0.set(get_node(union_find, right_root));
        } else {
            if left_rank == right_rank {
                left_root.1.set(left_rank + 1);
            }

            right_root.0.set(get_node(union_find, left_root));
        }
    }
}
