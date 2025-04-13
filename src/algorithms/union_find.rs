use std::cell::Cell;
use std::{mem, ptr};

type Node = (Cell<usize>, Cell<usize>);

pub struct UnionFind {
    nodes: Box<[Node]>,
}

impl UnionFind {
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes: vec![(Cell::new(usize::MAX), Cell::new(0)); nodes].into_boxed_slice(),
        }
    }

    pub fn get(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    fn get_index(&self, node: &Node) -> usize {
        (ptr::from_ref(node) as usize - self.nodes.as_ptr() as usize) / mem::size_of::<Node>()
    }

    pub fn find_root<'a>(&'a self, node: &'a Node) -> &'a Node {
        let parent = node.0.get();

        self.nodes.get(parent).map_or(node, |parent_node| {
            let root = self.find_root(parent_node);

            node.0.set(self.get_index(root));

            root
        })
    }

    pub fn union(&self, left_node: &Node, right_node: &Node) -> bool {
        let left_root = self.find_root(left_node);
        let right_root = self.find_root(right_node);

        if ptr::eq(left_root, right_root) {
            false
        } else {
            let left_rank = left_root.1.get();
            let right_rank = right_root.1.get();

            let (child, parent) = if left_rank < right_rank {
                (left_root, right_root)
            } else {
                if left_rank == right_rank {
                    left_root.1.set(left_rank + 1);
                }

                (right_root, left_root)
            };

            child.0.set(self.get_index(parent));

            true
        }
    }
}
