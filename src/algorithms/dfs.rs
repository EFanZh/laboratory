pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

pub fn preorder<T>(mut node: &Node<T>, mut f: impl FnMut(&Node<T>)) {
    let mut stack = Vec::new();

    loop {
        loop {
            f(node);

            node = if let Some(left) = node.left.as_deref() {
                if let Some(right) = &node.right {
                    stack.push(right.as_ref());
                }

                left
            } else if let Some(right) = node.right.as_deref() {
                right
            } else {
                break;
            };
        }

        if let Some(top) = stack.pop() {
            node = top;
        } else {
            break;
        }
    }
}

pub fn inorder<T>(mut node: &Node<T>, mut f: impl FnMut(&Node<T>)) {
    let mut stack = Vec::new();

    'outer: loop {
        while let Some(left) = node.left.as_deref() {
            stack.push(node);

            node = left
        }

        loop {
            f(node);

            if let Some(right) = node.right.as_deref() {
                node = right;

                break;
            }

            if let Some(top) = stack.pop() {
                node = top;
            } else {
                break 'outer;
            }
        }
    }
}

pub fn postorder<T>(mut node: &Node<T>, mut f: impl FnMut(&Node<T>)) {
    let mut stack = Vec::new();

    'outer: loop {
        loop {
            let (next, right) = match (node.left.as_deref(), node.right.as_deref()) {
                (None, None) => break,
                (None, Some(child)) | (Some(child), None) => (child, None),
                (Some(left), Some(right)) => (left, Some(right)),
            };

            stack.push((node, right));

            node = next;
        }

        loop {
            f(node);

            if let Some(top) = stack.last_mut() {
                if let Some(right) = top.1.take() {
                    node = right;

                    break;
                }

                node = top.0;

                stack.pop();
            } else {
                break 'outer;
            }
        }
    }
}
