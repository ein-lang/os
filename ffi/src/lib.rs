use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

extern "C" fn _ein_system_fd_write(fd: ffi::Number, buffer: ffi::EinString) -> ffi::Number {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    (file.write(buffer.as_slice()).unwrap() as f64).into()
}
