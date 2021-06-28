//! Library for interfacing with the ibstrace kernel module and parsing
//! AMD IBS micro-op samples.

pub mod msr;

pub mod ioctl {
    pub const IBSTRACE_CHARDEV: &str = "/dev/ibstrace";

    pub const CMD_WRITE:    usize = 0x0000_1000;
    pub const CMD_MEASURE:  usize = 0x0000_2000;
    pub const CMD_SAMPLES:  usize = 0x0000_4000;
    pub const CMD_CAPACITY: usize = 0x0000_8000;

    #[repr(C)]
    pub struct UserBuf { 
        // Pointer to buffer with user code
        ptr: *const u8, 
        // Buffer length
        len: usize 
    }
    impl UserBuf {
        pub fn new(ptr: *const u8, len: usize) -> Self {
            UserBuf { ptr, len }
        }
    }
}


/// A sample taken by the ibstrace kernel module.
#[derive(Clone, Default)]
#[repr(C)]
pub struct Sample {
    pub ctl:   msr::IbsOpCtl, 
    pub rip:   usize,
    pub data:  msr::IbsOpData, 
    pub data2: msr::IbsOpData2, 
    pub data3: msr::IbsOpData3, 
    pub linad: usize, 
    pub phyad: usize,
}


nix::ioctl_write_ptr_bad! {
    /// Submit code-to-be-measured to the kernel module. 
    /// Takes a pointer to a [UserBuf] describing the input buffer.
    ibstrace_write, ioctl::CMD_WRITE, ioctl::UserBuf
}

nix::ioctl_none_bad! {
    /// Execute the submitted user code and collect samples.
    ibstrace_measure, ioctl::CMD_MEASURE
}

nix::ioctl_none_bad! {
    /// Return the number of currently-collected samples.
    ibstrace_samples, ioctl::CMD_SAMPLES
}

nix::ioctl_none_bad! {
    /// Return the maximum number of entries in the sample buffer.
    ibstrace_capacity, ioctl::CMD_CAPACITY
}


/// Try to get a file descriptor for the ibstrace character device.
pub fn ibstrace_open() -> Result<i32, &'static str> {
    use nix::sys::stat::Mode;
    use nix::fcntl::{ open, OFlag };
    use nix::errno::Errno;
    use nix::Error;

    match open(ioctl::IBSTRACE_CHARDEV, OFlag::O_RDWR, Mode::S_IRWXU) {
        Ok(fd) => Ok(fd),
        Err(e) => match e {
            Error::Sys(eno) => match eno {
                Errno::ENOENT => Err("Kernel module not loaded?"),
                Errno::EACCES => Err("Permission denied - are you root?"),
                _ => panic!("{}", eno),
            },
            _ => panic!("unhandled error {}", e),
        }
    }
}

/// Read sample data back from the ibstrace character device.
pub unsafe fn ibstrace_read(fd: i32) -> Result<Box<[u8]>, &'static str> {
    use nix::unistd::read;
    let mut buf: Vec<u8>;

    // NOTE: Can I just use stat() to resolve the length of data available in 
    // the character device somehow? Instead of issuing more ioctls()?

    let samples = ibstrace_samples(fd).unwrap() as usize;
    let bytes = samples * std::mem::size_of::<Sample>();
    buf = vec![0; bytes];

    match read(fd, &mut buf){ 
        Ok(res) => {
            if res != bytes {
                return Err("Read unexpected number of bytes");
            } else {
                return Ok(buf.into_boxed_slice());
            }
        },
        Err(e) => panic!("{}", e),
    }
}

/// Close the file descriptor bound to the ibstrace character device.
pub fn ibstrace_close(fd: i32) {
    use nix::unistd::close;
    match close(fd) {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    }
}

