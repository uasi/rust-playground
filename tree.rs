struct Node {
    left: Option<Box<Node>>,
    //           ^^^ Required to declare recursive structure.
    right: Option<Box<Node>>,
    value: Option<int>,
}

fn make_leaf(value: Option<int>) -> Node {
    Node { left: None, right: None, value: value }
}

fn make_tree() -> Node {
    Node {
        left: Some(box Node {
            left: Some(box make_leaf(Some(30i))),
            right: None,
            value: Some(20i),
        }),
        right: Some(box make_leaf(Some(40i))),
        value: Some(10i),
    }
}

fn walk(node: &Node, depth: uint) {
    println!("{}{}", String::from_char(depth * 4u, ' '), node.value);
    //                                       ^^^^ While this must be uint,
    match node.left { //                      vvv this can be int.
        Some(ref node) => walk(&**node, depth + 1),
        //                     ^^^ Tedious?
        None => (),
    }
    match node.right {
        Some(box ref node) => walk(node, depth + 1),
        //   ^^^ Then add `box` here to kick sigils off.
        None => (),
    }
}

fn main() {
    let tree = make_tree();
    walk(&tree, 0);
}
