extern crate libc;

use std::mem;

struct Container {
    #[allow(dead_code)] value: i32,
}

impl Drop for Container {
    fn drop(&mut self) {
        let hello = vec![0x68i8, 0x65, 0x6c, 0x6c, 0x6f, 0x00];
        unsafe { libc::puts(hello.as_ptr()); }
    }
}

pub type ContainerPtr = *const Container;

#[no_mangle]
pub unsafe extern "C"
fn new_boxed_container() -> ContainerPtr {
    mem::transmute(Box::new(Container { value: 42 }))
}

#[no_mangle]
pub unsafe extern "C"
fn free_boxed_container(boxed_container: ContainerPtr) {
    let _: Box<Container> = mem::transmute(boxed_container);
}
