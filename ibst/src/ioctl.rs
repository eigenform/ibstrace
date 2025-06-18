
pub const IBSTRACE_CHARDEV: &str = "/dev/ibstrace";

pub const CMD_WRITE:    usize = 0x0000_1000;
pub const CMD_MEASURE:  usize = 0x0000_2000;
pub const CMD_SAMPLES:  usize = 0x0000_4000;
pub const CMD_CAPACITY: usize = 0x0000_8000;
pub const CMD_PRECISE:  usize = 0x0002_0000;

pub const MAX_OFFSET:   usize = 0x0010_0000;

#[repr(C)]
pub struct UserBuf { 
    // Pointer to buffer with user code
    ptr: *const u8, 
    // Buffer length
    len: usize,
}
impl UserBuf {
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self { ptr, len }
    }
}

#[repr(C)]
pub struct PreciseArgs { 
    // Pointer to buffer with user code
    ptr: *const u8, 
    // Offset
    offset: usize,
}
impl PreciseArgs {
    pub fn new(ptr: *const u8, offset: usize) -> Self {
        assert!(offset <= MAX_OFFSET,
            "Requested offset {} would exceeds maximum offset {}",
            offset, MAX_OFFSET
        );
        Self { ptr, offset }
    }
}

