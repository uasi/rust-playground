#![feature(unboxed_closures)]
#![feature(unsafe_destructor)]

macro_rules! defer {
    ($($body:stmt);* ;) => {
        let __deferred = Deferred::new(|| { $($body);* ; });
    }
}

struct Deferred<F> {
    finalize: F,
}

impl<F> Deferred<F> where F: FnMut() -> () {
    fn new(finalize: F) -> Deferred<F> {
        Deferred { finalize: finalize }
    }
}

#[unsafe_destructor]
impl<F> Drop for Deferred<F> where F: FnMut() {
    fn drop(&mut self) {
        #![allow(unstable)]
        self.finalize.call_mut(());
    }
}

fn hello() {
    defer! { println!("defer1"); }
    defer! { println!("defer2"); }

    print!("Hello, ");

    defer! { println!("defer3"); }

    println!("World!");
}

fn main() {
    hello();
    println!("done");
}
