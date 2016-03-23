use std::libc::{c_char,puts};
use std::c_str;

fn main() {
    let message = "Hello, world!".to_c_str();
    message.with_ref(|buf| {
        unsafe { puts(buf) };
    });
}
