// See http://mainisusuallyafunction.blogspot.jp/2014/08/calling-rust-library-from-c-or-anything.html
// to grab the gist of making a static library written in Rust.

#![no_std]
#![feature(macro_rules, phase, globs)]

// For Box.
extern crate alloc;

#[phase(plugin, link)]
extern crate core;

// For making fake `std`.
#[phase(plugin, link)]
extern crate collections;

// Without this it doesn't compile.
#[phase(plugin, link)]
extern crate log;

// For `puts`.
#[phase(plugin, link)]
extern crate libc;

// For ToCStr trait and its `with_c_str`.
#[phase(plugin, link)]
extern crate rustrt;

#[allow(unused_imports)]
use core::prelude::*;

use rustrt::c_str::*;

use alloc::boxed::Box;

/// A fake `std` module so that `deriving` and other macros will work.
/// See rust-lang/rust#16803.
mod std {
    pub use core::{clone, cmp, default, fmt, option, str};
    pub use collections::hash;
}

struct Container {
    #[allow(dead_code)] value: i32,
}

impl Drop for Container {
    fn drop(&mut self) {
        "dropping".with_c_str(|c_str| unsafe {
            libc::puts(c_str);
        });
    }
}

pub type ContainerPtr = *const ();

#[no_mangle]
pub unsafe extern "C"
fn new_boxed_container() -> ContainerPtr {
    core::mem::transmute(box Container { value: 42 })
}

#[no_mangle]
pub unsafe extern "C"
fn free_boxed_container(boxed_container: ContainerPtr) {
    let _: Box<Container> = core::mem::transmute(boxed_container);
}
