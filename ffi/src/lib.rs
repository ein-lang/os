mod result;

use bdwgc_alloc::Allocator;
use result::FfiResult;
use std::alloc::Layout;
use std::fs::File;
use std::io::{Read, Write};
use std::os::raw::{c_int, c_void};
use std::os::unix::io::FromRawFd;

extern "C" {
    fn _ein_system_main(argument: ffi::None) -> ffi::Number;
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
pub extern "C" fn _ein_malloc(size: usize) -> *mut c_void {
    (unsafe { std::alloc::alloc(Layout::from_size_align(size, 8).unwrap()) }) as *mut c_void
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    unsafe { Allocator::initialize() }

    f64::from(unsafe { _ein_system_main(ffi::None::new()) }) as c_int
}

#[no_mangle]
extern "C" fn _ein_system_fd_read(
    fd: ffi::Number,
    buffer_size: ffi::Number,
) -> *const FfiResult<ffi::EinString> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };
    let mut buffer = Vec::with_capacity(f64::from(buffer_size) as usize);

    if let Err(error) = file.read(&mut buffer) {
        return FfiResult::from_io_error(error);
    };

    std::mem::forget(file);

    FfiResult::ok(ffi::EinString::from(buffer))
}

#[no_mangle]
extern "C" fn _ein_system_fd_write(
    fd: ffi::Number,
    buffer: ffi::EinString,
) -> *const FfiResult<ffi::Number> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    let byte_count = match file.write(buffer.as_slice()) {
        Ok(count) => count,
        Err(error) => return FfiResult::from_io_error(error),
    };

    std::mem::forget(file);

    FfiResult::ok((byte_count as f64).into())
}
