use std::sync::Arc;

fn main() {
    let mut a1 = Arc::new(vec!["a1"]);
    let a2 = a1.clone();
    assert_eq!(a1, a2);
    a1 = Arc::new((*a1).clone());
    assert_eq!(a1, a2);
    // You can't mutate wrapped value:
    // a1.push("new");
    // (*mut a1).push("new");
}
