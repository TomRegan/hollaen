use std::libc::c_int;

#[link(name = "c")]
extern {
    fn printf(restrict: *u8, format: *u8) -> c_int;
}

fn main() {
    let len = unsafe {
        printf("%s\n".as_ptr(), "Hello, World!".as_ptr())
    };
    assert!(len == 14);
}
