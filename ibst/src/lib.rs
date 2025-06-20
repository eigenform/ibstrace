//! Library for interfacing with the ibstrace kernel module and parsing
//! AMD IBS micro-op samples.

#![feature(trait_alias)]
#![feature(pointer_is_aligned_to)]

pub mod ibs;
pub mod codegen;
pub mod util;
pub mod ioctl;
pub mod analysis;
pub mod msr; 

use std::hash::{Hash, Hasher};

/// A sample taken by the `ibstrace` kernel module.
///
/// WARNING: This struct mirrors the original definition in C code, see 
/// `ibstrace/ibstrace.h` in the source code for the `ibstrace` kernel module.
#[derive(Clone, Default, Ord, PartialOrd)]
#[repr(C)]
pub struct Sample {
    /// IBS OP sampling status register (IBS_OP_CTL).
    pub ctl:   ibs::IbsOpCtl, 
    /// Origin instruction pointer for this sample (IBS_OP_RIP).
    pub rip:   usize,
    /// Sample data (IBS_OP_DATA).
    pub data:  ibs::IbsOpData, 
    /// Sample data (IBS_OP_DATA2).
    pub data2: ibs::IbsOpData2, 
    /// Sample data (IBS_OP_DATA3).
    pub data3: ibs::IbsOpData3, 
    /// Linear address for tagged memory accesses (IBS_DC_LINADDR).
    pub linad: usize, 
    /// Physical address for tagged memory accesses (IBS_DC_PHYSADDR).
    pub phyad: usize,
    /// Sampled branch target address (BP_IBSTGT_RIP).
    pub tgt_rip: usize,
}
impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        self.rip == other.rip &&
        self.data == other.data &&
        self.data2 == other.data2 &&
        self.data3 == other.data3 && 
        self.linad == other.linad &&
        self.phyad == other.phyad &&
        self.tgt_rip == other.tgt_rip
    }
}
impl Eq for Sample {}

impl Hash for Sample {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rip.hash(state);
        self.data.hash(state);
        self.data2.hash(state);
        self.data3.hash(state);
        self.linad.hash(state);
        self.phyad.hash(state);
        self.tgt_rip.hash(state);
    }
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        fmt.debug_struct("Sample")
            .field("rip", &self.rip)
            .field("ld", &self.data3.ld_op())
            .field("st", &self.data3.st_op())
            .field("ucode", &self.data.op_microcode())
            .field("tag_to_retire", &self.data.tag_to_ret_ctr())
            .field("complete_to_retire", &self.data.comp_to_ret_ctr())
            .field("data_res_33", &self.data.res_33())
            .field("data_res_32", &self.data.res_32())
            .field("lin", &self.linad)
            .field("phy", &self.phyad)
            .field("width", &self.data3.op_mem_width())
            .finish()
    }
}


nix::ioctl_write_ptr_bad! {
    /// Submit code-to-be-measured to the kernel module. 
    /// Takes a pointer to a [UserBuf] describing the input buffer.
    ibstrace_write, ioctl::CMD_WRITE, ioctl::UserBuf
}

nix::ioctl_write_ptr_bad! {
    /// Execute submitted user code in the "precise" environment. 
    /// Takes a pointer to a [PreciseArgs] describing the arguments.
    ibstrace_precise, ioctl::CMD_PRECISE, ioctl::PreciseArgs
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

    match open(ioctl::IBSTRACE_CHARDEV, OFlag::O_RDWR, Mode::S_IRWXU) {
        Ok(fd) => Ok(fd),
        Err(e) => match e {
            Errno::ENOENT => Err("Kernel module not loaded?"),
            Errno::EACCES => Err("Permission denied - are you root?"),
            _ => panic!("{}", e),
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

/// Return the base address of the code buffer.
///
/// NOTE: This is not very pretty, but I didn't want to add more ioctls.
pub fn get_base_address() -> Result<usize, &'static str> {
    use std::fs::read_to_string;
    match read_to_string("/sys/kernel/debug/ibstrace/code_buf") {
        Ok(s) => {
            let x = s[2..].strip_suffix("\n").unwrap();
            Ok(usize::from_str_radix(x, 16).unwrap())
        }
        Err(e) => panic!("Couldn't read ibstrace base address? - {}", e),
    }
}

/// Execute and sample some code, returning a [Box] of [Sample] data.
pub fn measure(fd: i32, msg: &ioctl::UserBuf) -> Box<[Sample]> {
    unsafe { 
        match ibstrace_write(fd, msg as *const ioctl::UserBuf) {
            Ok(res) => if res < 0 { 
                panic!("write failed with {}", res);
            },
            Err(e) => panic!("{}", e),
        }
        match ibstrace_measure(fd) { 
            Ok(res) => if res < 0 { 
                panic!("measure failed with {}", res);
            },
            Err(e) => panic!("{}", e),
        }
        let data = match ibstrace_read(fd) {
            Ok(buf) => buf,
            Err(e) => panic!("{}", e),
        };

        std::slice::from_raw_parts(
            data.as_ptr() as *mut Sample,
            data.len() / std::mem::size_of::<Sample>()
        ).to_owned().into_boxed_slice()
    }
}

/// Execute and sample some code, returning a [Box] of [Sample] data.
pub fn measure_precise(fd: i32, msg: &ioctl::UserBuf, arg: &ioctl::PreciseArgs) 
    -> Box<[Sample]> 
{
    unsafe { 
        match ibstrace_write(fd, msg as *const ioctl::UserBuf) {
            Ok(res) => if res < 0 { 
                panic!("write failed with {}", res);
            },
            Err(e) => panic!("{}", e),
        }
        match ibstrace_precise(fd, arg as *const ioctl::PreciseArgs) {
            Ok(res) => if res < 0 { 
                panic!("precise measurement failed with {}", res);
            },
            Err(e) => panic!("{}", e),
        }
        let data = match ibstrace_read(fd) {
            Ok(buf) => buf,
            Err(e) => panic!("{}", e),
        };

        std::slice::from_raw_parts(
            data.as_ptr() as *mut Sample,
            data.len() / std::mem::size_of::<Sample>()
        ).to_owned().into_boxed_slice()
    }
}


