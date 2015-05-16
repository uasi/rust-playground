#![feature(unboxed_closures)]

use std::ops::{Deref, DerefMut};

struct Guard<T, F> where F: FnMut(&mut T) {
    object: T,
    finally: F,
}

impl<T, F> Guard<T, F> where F: FnMut(&mut T) {
    fn new(object: T, finally: F) -> Guard<T, F> {
        Guard { object: object, finally: finally }
    }
}

impl<T, F> Deref for Guard<T, F> where F: FnMut(&mut T) {
    type Target = T;
    fn deref(&self) -> &T {
        &self.object
    }
}

impl<T, F> DerefMut for Guard<T, F> where F: FnMut(&mut T) {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.object
    }
}

impl<T, F> Drop for Guard<T, F> where F: FnMut(&mut T) {
    fn drop(&mut self) {
        #![allow(unstable)]
        self.finally.call_mut((&mut self.object,));
    }
}

#[derive(Debug)]
struct Guarded {
    void: ()
}

impl Guarded {
    fn greet(&self) {
        println!("Hello, world!");
    }

    fn kill(&mut self) {
        println!("Killed");
    }
}

fn hello() {
    println!("Entered hello");
    let g = Guard::new(Guarded {void: () }, |g| {
        println!("Killing guarded");
        g.kill();
    });
    g.greet();
    println!("Leaving hello");
}

fn main() {
    hello();
    println!("Done");
}
