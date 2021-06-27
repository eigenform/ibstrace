
extern crate nix;
pub mod msr;

use std::env;
use std::fs::File;
use std::io::Read;

use nix::Error;
use nix::errno::Errno;
use nix::sys::stat::Mode;
use nix::fcntl::{ open, OFlag };
use nix::unistd::{ read, close };

use crate::msr::*;

/// Path to the ibstrace character device
pub const IBSTRACE_CHARDEV: &str = "/dev/ibstrace";

/// The maximum capacity of the kernel sample buffer
pub const IBSTRACE_SAMPLE_CAPACITY: usize = 256;

pub const IBSTRACE_CMD_WRITE: usize = 0x1000;
pub const IBSTRACE_CMD_MEASURE: usize = 0x2000;
pub const IBSTRACE_CMD_NUM_SAMPLE: usize = 0x4000;

// Declare wrapper functions for the different ioctls
nix::ioctl_write_ptr_bad!(ibstrace_write, IBSTRACE_CMD_WRITE, Message);
nix::ioctl_none_bad!(ibstrace_measure, IBSTRACE_CMD_MEASURE);
nix::ioctl_none_bad!(ibstrace_num_sample, IBSTRACE_CMD_NUM_SAMPLE);

#[repr(C)]
pub struct Message { ptr: *const u8, len: usize }

#[derive(Clone, Default)]
#[repr(C)]
pub struct Sample {
    ctl: IbsOpCtl, 
    rip: usize,
    data: IbsOpData, 
    data2: IbsOpData2, 
    data3: IbsOpData3, 
    data4: IbsOpData4,
    linad: usize, 
    phyad: usize,
    tid: u32, pid: u32, cpu: u32, krn: u32,
}

/// Try to get a file descriptor for the character device.
fn open_device() -> Result<i32, &'static str> {
    match open(IBSTRACE_CHARDEV, OFlag::O_RDWR, Mode::S_IRWXU) {
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

fn measure_code(fd: i32, code: &[u8], len: usize) -> Result<Box<[u8]>, String> {
    let mut res: i32;
    let mut outbuf: Vec<u8>;
    let num_sample: i32;
    let num_bytes: usize;

    let msg = Message { ptr: code.as_ptr(), len };
    unsafe {
        res = ibstrace_write(fd, &msg as *const Message).unwrap();
        if res < 0 { 
            return Err(format!("write ioctl returned {}", res));
        }

        res = ibstrace_measure(fd).unwrap();
        if res < 0 {
            return Err(format!("measure ioctl returned {}", res));
        }

        num_sample = ibstrace_num_sample(fd).unwrap();
        if num_sample <= 0 {
            return Err("No samples collected".to_string());
        }
        if num_sample > IBSTRACE_SAMPLE_CAPACITY as i32 {
            return Err(format!("Exceeded sample capacity ({})", res));
        }

        num_bytes = num_sample as usize * std::mem::size_of::<Sample>();
        println!("Collected {} samples ({} bytes)", num_sample, num_bytes);

        // Actually allocate and read the samples
        outbuf = vec![0; num_bytes];
        if read(fd, &mut outbuf).unwrap() != num_bytes {
            return Err("failed to read sample data".to_string());
        }
    }
    Ok(outbuf.into_boxed_slice())
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <file>", args[0]);
        return;
    }

    // Read user input
    let mut input = File::open(&args[1]).expect("");
    let input_len = std::fs::metadata(&args[1]).unwrap().len() as usize;
    let mut input_buffer = vec![0; input_len];
    input.read(&mut input_buffer).unwrap();


    let fd = match open_device() {
        Ok(fd) => fd,
        Err(e) => panic!("{}", e),
    };

    let res = measure_code(fd, &input_buffer, input_len);
    close(fd).unwrap();
    let sample_data = match res {
        Ok(buf) => buf,
        Err(e) => panic!("{}", e),
    };



    //let output = File::create("/tmp/foo.bin").unwrap();


}



