extern crate libc;
extern crate ncurses;

use libc::{c_int};
use ncurses as nc;
use std::ffi::CString;
use std::thread;

mod native {
    use libc::{c_char, c_int};
    extern {
        pub fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
    }
}
fn main() {
    unsafe {
        let locale = CString::new("").unwrap();
        native::setlocale(0 as c_int, locale.as_ptr());
    }

    nc::initscr();

    let spinner = "⠋⠙⠸⠴⠦⠇";
//    let spinner = "⠃⠉⠘⠰⠤⠆";
//    let spinner = "⠟⠻⠽⠾⠷⠯";
//    let spinner = "⠁⠈⠐⠠⠄⠂";
//    let spinner = "⠁⠈⠐⠂⠄⠠⠐⠂";
//    let spinner = r"|/-\";

    for ch in spinner.chars().cycle() {
        nc::clear();
        nc::mvaddstr(0, 0, format!("{}", ch).as_ref());
        nc::refresh();
        thread::sleep_ms(200);
    }

    nc::endwin();
}
