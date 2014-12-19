fn main() {
    let mut i = 42i32;
    f(&mut i);
}

fn f(i: &mut i32) {
    if false { drop(i); }
    println!("{}", i);
    //             ^ error: use of moved value
}
