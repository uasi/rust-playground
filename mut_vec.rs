#[deriving(Show)]
struct Item {
    value: int
}

impl Item {
    fn new(value: int) -> Item {
        Item { value: value }
    }
}

fn make_vec() -> Vec<Vec<Item>> {
    let mut v = Vec::new();
    for i in range(0i, 5) {
        let mut w = Vec::new();
        for j in range(0i, 5) {
            w.push(Item::new(i * 10 + j));
        }
        v.push(w);
    }
    v
}

fn reject_odd_items(v: &mut Vec<Vec<Item>>) {
    for w in v.iter_mut() {
        w.retain(|i| i.value % 2 == 1);
    }
}

fn double_item_value(v: &mut Vec<Vec<Item>>) {
    for w in v.iter_mut() {
        for i in w.iter_mut() {
            i.value *= 2;
        }
    }
}

fn main() {
    let mut v = make_vec();
    reject_odd_items(&mut v);
    double_item_value(&mut v);
    println!("{}", v);
}
