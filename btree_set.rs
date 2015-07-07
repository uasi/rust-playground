use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    set.insert(0usize);
    set.insert(199);
    set.insert(25);
    set.insert(82);
    set.insert(13);
    set.insert(999);
    for i in set.iter() {
        println!("{}", i); // numbers will be printed in ascending order
    }
}
