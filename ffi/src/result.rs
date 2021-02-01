#[repr(C)]
pub struct FfiResult<T: Default> {
    value: T,
    error: ffi::Number,
}

impl<T: Default> FfiResult<T> {
    pub fn ok(value: T) -> *const Self {
        Self {
            value,
            error: 0.0.into(),
        }
        .leak()
    }

    pub fn error(error: impl Into<ffi::Number>) -> *const Self {
        Self {
            value: Default::default(),
            error: error.into(),
        }
        .leak()
    }

    pub fn from_io_error(error: std::io::Error) -> *const Self {
        Self::error(error.raw_os_error().map(f64::from).unwrap_or(std::f64::NAN))
    }

    fn leak(self) -> *const Self {
        Box::leak(Box::new(self))
    }
}
