extern crate libc;

use libc::{c_ulong, c_uint};
use std::ptr;

mod z {
    use libc::{c_ulong, c_uint};

    #[link(name = "z")]
    extern {
        pub fn adler32(adler: c_ulong, buf: *const u8, len: c_uint) -> c_ulong;
    }
}

fn adler32(adler: u64, buf: &[u8]) -> u64 {
    unsafe {
        match buf.len() {
            0 => z::adler32(adler as c_ulong, ptr::null(), 0 as c_uint) as u64,
            _ => z::adler32(adler as c_ulong, buf.as_ptr(), buf.len() as c_uint) as u64
        }
    }
}

fn main() {
    let adler = adler32(0, &[]);
    println!("initial value: {}", adler);
    let adler = adler32(adler, &[1u8, 2u8, 3u8]);
    println!("put [1, 2, 3]: {}", adler);
}
