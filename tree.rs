use std::iter::{self, FromIterator};

struct Node {
    left: Option<Box<Node>>,
    //           ^^^ Required to declare recursive structure.
    right: Option<Box<Node>>,
    value: Option<i32>,
}

fn make_leaf(value: Option<i32>) -> Node {
    Node { left: None, right: None, value: value }
}

fn make_tree() -> Node {
    Node {
        left: Some(Box::new(Node {
            left: Some(Box::new(make_leaf(Some(30)))),
            right: None,
            value: Some(20),
        })),
        right: Some(Box::new(make_leaf(Some(40)))),
        value: Some(10),
    }
}

fn walk(node: &Node, depth: u32) {
    let indent: String = FromIterator::from_iter(iter::repeat("    ").take(depth as usize));
    println!("{}{:?}", indent, node.value);
    match node.left {
        Some(ref node) => walk(&**node, depth + 1),
        None => (),
    }
    match node.right {
        Some(ref node) => walk(&**node, depth + 1),
        None => (),
    }
}

fn main() {
    let tree = make_tree();
    walk(&tree, 0);
}
