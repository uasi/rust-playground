// See http://mainisusuallyafunction.blogspot.jp/2014/08/calling-rust-library-from-c-or-anything.html
// to grab the gist of making a static library written in Rust.

#![no_std]
#![feature(plugin)]

// For `Box`.
extern crate alloc;

#[plugin]
extern crate core;

// For making fake `std`.
#[plugin]
#[macro_use(vec)]
extern crate collections;

// Without this it doesn't compile.
#[plugin]
extern crate log;

// For `puts`.
#[plugin]
extern crate libc;

#[allow(unused_imports)]
use core::prelude::*;

use alloc::boxed::Box;

/// A fake `std` module so that `derive` and other macros will work.
/// See rust-lang/rust#16803.
mod std {
    pub use core::{clone, cmp, default, fmt, option, str};
}

struct Container {
    #[allow(dead_code)] value: i32,
}

impl Drop for Container {
    fn drop(&mut self) {
        let hello = vec![0x68i8, 0x65, 0x6c, 0x6c, 0x6f, 0x00];
        unsafe { libc::puts(core::mem::transmute(hello.as_ptr())); }
    }
}

pub type ContainerPtr = *const ();

#[no_mangle]
pub unsafe extern "C"
fn new_boxed_container() -> ContainerPtr {
    core::mem::transmute(Box::new(Container { value: 42 }))
}

#[no_mangle]
pub unsafe extern "C"
fn free_boxed_container(boxed_container: ContainerPtr) {
    let _: Box<Container> = core::mem::transmute(boxed_container);
}
