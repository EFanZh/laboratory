use std::cell::Cell;
use std::{mem, ptr};

type State = (Cell<usize>, Cell<usize>);

pub struct UnionFind {
    states: Box<[State]>,
}

impl UnionFind {
    pub fn new(nodes: usize) -> Self {
        Self {
            states: vec![(Cell::new(usize::MAX), Cell::new(0)); nodes].into_boxed_slice(),
        }
    }

    fn get_node(states: &[State], state: &State) -> usize {
        (ptr::from_ref(state) as usize - states.as_ptr() as usize) / mem::size_of::<State>()
    }

    fn find_root_inner<'a>(states: &'a [State], state: &'a State) -> &'a State {
        let parent = state.0.get();

        states.get(parent).map_or(state, |parent_state| {
            let root = Self::find_root_inner(states, parent_state);

            state.0.set(Self::get_node(states, root));

            root
        })
    }

    fn union_inner(states: &[State], left_state: &State, right_state: &State) {
        let left_root = Self::find_root_inner(states, left_state);
        let right_root = Self::find_root_inner(states, right_state);

        if !ptr::eq(left_root, right_root) {
            let left_rank = left_root.1.get();
            let right_rank = right_root.1.get();

            if left_rank < right_rank {
                left_root.0.set(Self::get_node(states, right_root));
            } else {
                if left_rank == right_rank {
                    left_root.1.set(left_rank + 1);
                }

                right_root.0.set(Self::get_node(states, left_root));
            }
        }
    }

    pub fn get_root(&self, node: usize) -> usize {
        let state = Self::find_root_inner(&self.states, &self.states[node]);

        Self::get_node(&self.states, state)
    }

    pub fn union(&self, left: usize, right: usize) {
        let left_state = &self.states[left];
        let right_state = &self.states[right];

        Self::union_inner(&self.states, left_state, right_state);
    }
}
