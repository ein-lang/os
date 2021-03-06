#[repr(C)]
#[derive(Clone, Copy)]
pub struct Stack {
    base_pointer: *mut u8,
    size: usize,
    capacity: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Result {}
