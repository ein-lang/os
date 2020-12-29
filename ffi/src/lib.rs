use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::process::exit;

#[no_mangle]
extern "C" fn _ein_system_fd_write(fd: ffi::Number, buffer: ffi::EinString) -> ffi::Number {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    (file.write(buffer.as_slice()).unwrap() as f64).into()
}

#[no_mangle]
extern "C" fn _ein_system_exit(code: ffi::Number) -> ffi::None {
    exit(f64::from(code) as i32)
}
