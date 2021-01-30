use bdwgc_alloc::Allocator;
use std::fs::File;
use std::io::Write;
use std::os::raw::c_int;
use std::os::unix::io::FromRawFd;

extern "C" {
    fn _ein_system_main(argument: ffi::None) -> ffi::Number;
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
pub extern "C" fn main() -> c_int {
    unsafe { Allocator::initialize() }

    f64::from(unsafe { _ein_system_main(ffi::None::new()) }) as c_int
}

#[no_mangle]
extern "C" fn _ein_system_fd_write(fd: ffi::Number, buffer: ffi::EinString) -> ffi::Number {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    (file.write(buffer.as_slice()).unwrap() as f64).into()
}
