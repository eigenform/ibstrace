//! For dynamically generating some code to-be-measured during runtime.
//!
//! # Runtime Environment
//! The trampoline in the `ibstrace` kernel module uses `CALL` to enter some
//! measured code. Any generated code should terminate with a `RET`.
//!
//! The kernel module passes a single parameter to measured code in RDI - as 
//! of right now, this is the physical address of some freshly-allocated page
//! which is free-for-use by the measured code. Otherwise, assume the initial 
//! state of all other registers is undefined. The current design is not 
//! intended to support measured code with stack usage, and I haven't thought 
//! about it at all yet. 
//!
//! ## Safety
//! This entire thing is *highly unsafe by design*, and there's practically no
//! avoiding it - if you do something wrong in measured code, you *will* crash 
//! in the context of the kernel. 
//!
//! Eventually I'll think of some way to catch faulting instructions and 
//! restore execution in a way that doesn't involve hard-resetting my machine,
//! but that would involve some cooperation with the kernel module.
//!
//! ## Writing tests
//! The [emit_test_iters_rsi!()] macro is suitable for emitting simple loops,
//! which you can use to test individual instructions.
//!

use dynasmrt::{ 
    dynasm, 
    DynasmApi, 
    DynasmLabelApi,
    Assembler, 
    AssemblyOffset, 
    ExecutableBuffer, 
    x64::X64Relocation,
};

// NOP encodings from length 1-15 (single instructions).
//
// See AMD publication 56305 (Rev. 3.00), "Software Optimization Guide for AMD 
// Family 17h Models 30h and Greater Processors", from section 2.8.3.1 
// "Encoding Padding for Loop Alignment", page 29.

const NOP1: [u8;1] = [ 0x90 ];
const NOP2: [u8;2] = [ 0x66, 0x90 ];
const NOP3: [u8;3] = [ 0x0F, 0x1F, 0x00 ];
const NOP4: [u8;4] = [ 0x0F, 0x1F, 0x40, 0x00 ];
const NOP5: [u8;5] = [ 0x0F, 0x1F, 0x44, 0x00, 0x00 ];
const NOP6: [u8;6] = [ 0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00 ];
const NOP7: [u8;7] = [ 0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00 ];
const NOP8: [u8;8] = [ 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00 ];
const NOP9: [u8;9] = [ 0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00 ];
const NOP10: [u8;10] = [ 
    0x66, 0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00 
];
const NOP11: [u8;11] = [ 
    0x66, 0x66, 0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00 
];
const NOP12: [u8;12] = [ 
    0x66, 0x66, 0x66, 0x66, 0x0F, 0x1F, 0x84, 
    0x00, 0x00, 0x00, 0x00, 0x00 
];
const NOP13: [u8;13] = [ 
    0x66, 0x66, 0x66, 0x66, 0x66, 0x0F, 0x1F, 0x84, 
    0x00, 0x00, 0x00, 0x00, 0x00 
];
const NOP14: [u8;14] = [ 
    0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x0F, 0x1F, 0x84, 
    0x00, 0x00, 0x00, 0x00, 0x00
];
const NOP15: [u8;15] = [ 
    0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x0F, 0x1F, 0x84, 
    0x00, 0x00, 0x00, 0x00, 0x00
];





/// Description of the code generated/assembled for a particular test.
///
/// NOTE: Right now, this model assumes that we're interested in samples
/// for a single "target" instruction within the stream.
///
pub struct TestParameters {
    /// Buffer with code for this test.
    pub buf: ExecutableBuffer,
    /// Offset to a single "target" instruction within the buffer.
    pub tgt_instr_off: usize,
}
impl TestParameters {
    /// Create the appropriate ioctl message for this test.
    pub fn to_userbuf(&self) -> crate::ioctl::UserBuf {
        crate::ioctl::UserBuf::new(
            self.buf.ptr(AssemblyOffset(0)), 
            self.buf.len(),
        )
    }
}


#[macro_export]
macro_rules! emit_test {
    ($num_iter:expr, {$($pre:tt)*}, {$($body:tt)*}) => { {
        let mut asm = Assembler::<X64Relocation>::new().unwrap();
        dynasm!(asm
            ; .arch x64

            $($pre)*

            ; mov   rsi, $num_iter as _
            ; .align 64
            ; ->loop_start:

            $($body)*

            ; sub   rsi, 1
            ; jne   ->loop_start

            ; mov   rax, 42
            ; ret
        );

        let offset = match asm.labels().resolve_global("target") {
            Ok(offset) => offset.0,
            Err(e) => panic!("{:?}", e),
        };

        let buf = asm.finalize().unwrap();
        TestParameters { buf, tgt_instr_off: offset }
    } }
}

#[cfg(test)]
mod test {
    use crate::codegen::*;
    use crate::util::*;
    #[test]
    fn test() {
        let msr: u32 = 0xc0010200;
        let t = emit_test!(0x1000,
            {
                ; mov ecx, msr as _
                ; xor ecx, ecx
            },
            {
                ; mov ecx, msr as _
                ; ->target:
                ; rdmsr
            }
        );
        disas(&t.buf);
    }

}



/// Wrapper around dynasm for emitting a simple loop (decrementing RSI).
///
/// **WARNING:** When using this macro, you *must* create a global label named
/// "target" - otherwise, this will panic when we fail to unwrap the offset
/// to the target instruction during runtime. 
///
#[macro_export]
macro_rules! emit_test_iters_rsi {
    ($num_iter:expr, $($t:tt)*) => { {
        let mut asm = Assembler::<X64Relocation>::new().unwrap();
        dynasm!(asm
            ; .arch x64
            ; mov   rsi, $num_iter as _
            ; ->loop_start:

            $($t)*

            ; sub   rsi, 1
            ; jne   ->loop_start
            ; mov   rax, 42
            ; ret
        );

        let offset = match asm.labels().resolve_global("target") {
            Ok(offset) => offset.0,
            Err(e) => panic!("{:?}", e),
        };

        let buf = asm.finalize().unwrap();
        TestParameters { buf, tgt_instr_off: offset }
    } }
}

pub fn emit_msr_test(msr: u32, iters: usize) -> TestParameters {
    emit_test_iters_rsi!(iters, 
        ; mov   ecx, msr as _
        ; ->target:
        ; rdmsr
    )
}
pub fn emit_cpuid_test(cpuid_func: u32, iters: usize) -> TestParameters {
    emit_test_iters_rsi!(iters,
        ; mov   eax, cpuid_func as _
        ; ->target:
        ; cpuid
    )
}

