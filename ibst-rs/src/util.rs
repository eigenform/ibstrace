use nix;
use nix::sys::mman::{ ProtFlags, MapFlags };
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};
use std::os::raw::c_void;
use std::num::NonZero;
use std::ptr::NonNull;

pub fn disas(buf: &[u8]) {
    use iced_x86::{
        Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter
    };
    let mut decoder = Decoder::with_ip(64, buf, 0, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();

    // Change some options, there are many more
    formatter.options_mut().set_digit_separator("_");
    formatter.options_mut().set_first_operand_char_index(10);

    let mut output = String::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        output.clear();
        formatter.format(&instruction, &mut output);

        print!("{:04X} ", instruction.ip());
        let start_index = (instruction.ip() - 0) as usize;
        let instr_bytes = &buf[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02X}", b);
        }
        if instr_bytes.len() < 10 {
            for _ in 0..10 - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", output);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PageMapEntry(pub u64);
impl PageMapEntry {
    const NUM_BYTES: usize = 8;
    pub fn present(&self) -> bool {
        (self.0 & (1 << 63)) != 0
    }
    pub fn swapped(&self) -> bool {
        (self.0 & (1 << 62)) != 0
    }
    pub fn exclusive(&self) -> bool {
        (self.0 & (1 << 56)) != 0
    }
    pub fn soft_dirty(&self) -> bool {
        (self.0 & (1 << 55)) != 0
    }
    pub fn pfn(&self) -> usize {
        self.0 as usize & ((1 << 55) - 1)
    }
}


pub struct PageMap;
impl PageMap { 
    // Assume 4KiB pagesize
    const NUM_OFFSET_BITS: usize = 12;

    pub fn resolve_paddr(vaddr: usize) -> Result<usize, &'static str> { 
        use std::io::prelude::*;
        let mut f = std::fs::File::open("/proc/self/pagemap").map_err(|_| { 
            "Couldn't open /proc/self/pagemap (do you have permission?)"
        })?;

        // Seek to the appropriate pagemap entry and read it
        let mut buf = [0u8; 8];
        let vfn  = vaddr >> Self::NUM_OFFSET_BITS;
        let foff = (vfn * PageMapEntry::NUM_BYTES) as u64;
        f.seek(std::io::SeekFrom::Start(foff)).unwrap();
        f.read_exact(&mut buf).unwrap();
        let entry = PageMapEntry(u64::from_le_bytes(buf));

        if !entry.present() {
            return Err("Couldn't find entry in page map");
        }
        if entry.pfn() == 0 {
            return Err("Got PFN 0 from page map (do you have permission?)");
        }

        // Compute the physical address
        let paddr = (
            (entry.pfn() << Self::NUM_OFFSET_BITS) | 
            (vaddr & ((1 << Self::NUM_OFFSET_BITS) - 1))
        );

        Ok(paddr)
    }
}


/// A physically-contiguous memory allocation. 
pub struct RawAlloc { 
    /// Pointer returned from call to `mmap()`
    pub ptr: NonNull<c_void>,

    /// The physical address corresponding to the pointer
    pub paddr: usize,

    /// The length of this allocation (in bytes)
    pub len: usize,
}
impl RawAlloc { 

    /// System page size is 4KiB
    const PAGE_SIZE: usize = (1 << 12);

    /// Default PMD size (2MiB)
    const DEFAULT_SZ: usize = (1 << 21);

    /// Try to request a 2MiB physically-contiguous memory allocation. 
    ///
    /// If transparent hugepages (THP) is enabled, this *should* allocate a 
    /// set of physically-contiguous pages for us. 
    ///
    /// FIXME: We're just panicking when mmap fails
    unsafe fn alloc() -> Result<(NonNull<c_void>, usize), &'static str> { 
        let len = Self::DEFAULT_SZ;
        let ptr = unsafe {
            nix::sys::mman::mmap_anonymous(None, NonZero::new(len).unwrap(),
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_ANONYMOUS | MapFlags::MAP_PRIVATE | MapFlags::MAP_LOCKED
            ).map_err(|_| "Call to mmap() failed (?)")?
        };
        Ok((ptr, len))
    }

    pub fn new() -> Result<Self, &'static str> { 

        // Allocate 2MiB
        let (vptr_base, len) = unsafe { Self::alloc()? };
        assert!(vptr_base.is_aligned_to(Self::DEFAULT_SZ));

        // Resolve the physical address
        let paddr_base = PageMap::resolve_paddr(vptr_base.addr().get())?;
        let pptr_base  = NonNull::new(paddr_base as *mut c_void).unwrap();
        assert!(pptr_base.is_aligned_to(Self::DEFAULT_SZ));

        // Validate that all 4KiB chunks are physically-contiguous.
        // NOTE: Patches are small and we really only need the first few.
        for idx in 0..(Self::DEFAULT_SZ / Self::PAGE_SIZE) {
            let off = idx * Self::PAGE_SIZE;
            let vaddr = vptr_base.addr().get() + off;
            let paddr = PageMap::resolve_paddr(vaddr)?;
            if paddr != (paddr_base + off) {
                return Err("Failed to allocate physically-contiguous memory");
            }
        }

        Ok(Self { ptr: vptr_base, paddr: paddr_base, len })
    }

    /// Return the virtual address as a `usize`.
    pub fn addr(&self) -> usize { 
        self.ptr.addr().get()
    }

    /// Return the physical address as a `usize`.
    pub fn paddr(&self) -> usize { 
        self.paddr
    }

    pub fn ptr(&self) -> NonNull<u8> { 
        self.ptr.cast()
    }

}

impl Drop for RawAlloc { 
    fn drop(&mut self) {
        unsafe { 
            nix::sys::mman::munmap(self.ptr, self.len).unwrap()
        }
    }
}

