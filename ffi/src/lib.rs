mod result;

use result::FfiResult;
use std::alloc::Layout;
use std::fs::File;
use std::io::{Read, Write};
use std::os::raw::{c_int, c_void};
use std::os::unix::io::FromRawFd;

const INITIAL_STACK_CAPACITY: usize = 256;

extern "C" {
    fn _ein_os_main(
        stack: *mut ffi::cps::Stack,
        continuation: extern "C" fn(*mut ffi::cps::Stack, f64) -> ffi::cps::Result,
        argument: ffi::None,
    ) -> ffi::cps::Result;
}

const DEFAULT_ALIGNMENT: usize = 8;

#[no_mangle]
pub extern "C" fn _ein_malloc(size: usize) -> *mut c_void {
    (unsafe { std::alloc::alloc(Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap()) })
        as *mut c_void
}

#[no_mangle]
pub extern "C" fn _ein_realloc(pointer: *mut c_void, size: usize) -> *mut c_void {
    // Layouts are expected to be ignored by the global allocator.
    (unsafe {
        std::alloc::realloc(
            pointer as *mut u8,
            Layout::from_size_align(0, DEFAULT_ALIGNMENT).unwrap(),
            size,
        )
    }) as *mut c_void
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    let mut stack = ffi::cps::Stack::new(INITIAL_STACK_CAPACITY);

    unsafe { _ein_os_main(&mut stack, exit, ffi::None::new()) };

    unreachable!()
}

extern "C" fn exit(_: *mut ffi::cps::Stack, code: f64) -> ffi::cps::Result {
    std::process::exit(code as i32)
}

#[no_mangle]
extern "C" fn _ein_os_fd_read(
    fd: ffi::Number,
    buffer_size: ffi::Number,
) -> ffi::Arc<FfiResult<ffi::EinString>> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };
    let mut buffer = vec![0; f64::from(buffer_size) as usize];

    let count = match file.read(&mut buffer) {
        Ok(count) => count,
        Err(error) => return ffi::Arc::new(error.into()),
    };
    buffer.resize(count, 0);

    std::mem::forget(file);

    FfiResult::ok(ffi::EinString::from(buffer)).into()
}

#[no_mangle]
extern "C" fn _ein_os_fd_write(
    fd: ffi::Number,
    buffer: ffi::EinString,
) -> ffi::Arc<FfiResult<ffi::Number>> {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    let count = match file.write(buffer.as_slice()) {
        Ok(count) => count,
        Err(error) => return ffi::Arc::new(error.into()),
    };

    std::mem::forget(file);

    FfiResult::ok((count as f64).into()).into()
}
