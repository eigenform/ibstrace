
/// Path to the 'ibstrace' character device.
pub const IBSTRACE_CHARDEV: &str = "/dev/ibstrace";

/// The "write" ioctl() command
pub const CMD_WRITE:    usize = 0x0000_1000;

/// The "measure" ioctl() command
pub const CMD_MEASURE:  usize = 0x0000_2000;

/// The "samples" ioctl() command
pub const CMD_SAMPLES:  usize = 0x0000_4000;

/// The "capacity" ioctl() command
pub const CMD_CAPACITY: usize = 0x0000_8000;

/// The "precise" ioctl() command
pub const CMD_PRECISE:  usize = 0x0002_0000;

/// The maximum supported offset in "precise" sampling mode. 
///
/// NOTE: This is also defined as a constant in the kernel module. 
/// This *must* match the definition in `include/asm/ibstrace_asm.h`. 
pub const MAX_OFFSET:   usize = 0x0040_0000;

/// Argument to [`CMD_WRITE`], used to upload user code. 
#[repr(C)]
pub struct UserBuf { 
    /// Pointer to buffer with user code
    ptr: *const u8, 
    /// Buffer length
    len: usize,
}
impl UserBuf {
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self { ptr, len }
    }
}

/// Argument to [`CMD_PRECISE`], used to sample a particular micro-op. 
#[repr(C)]
pub struct PreciseArgs { 
    /// Argument passed through to user code (in RDI)
    arg: usize,
    /// Counter offset
    offset: usize,
}
impl PreciseArgs {
    pub fn new(arg: usize, offset: usize) -> Self {
        assert!(offset <= MAX_OFFSET,
            "Requested offset {} would exceeds maximum offset {}",
            offset, MAX_OFFSET
        );
        Self { arg, offset }
    }
}

