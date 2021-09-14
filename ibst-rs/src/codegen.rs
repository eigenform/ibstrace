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

use dynasmrt::{ 
    dynasm, 
    DynasmApi, 
    DynasmLabelApi,
    Assembler, 
    AssemblyOffset, 
    ExecutableBuffer, 
    x64::X64Relocation,
};

/// Wrapper around dynasm for emitting a simple loop (decrementing RSI).
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
        asm.finalize().unwrap()
    } }
}

pub fn emit_msr_test(msr: u32, iters: usize) -> ExecutableBuffer {
    emit_test_iters_rsi!(iters, 
        ; mov   ecx, msr as _
        ; rdmsr
    )
}
pub fn emit_cpuid_test(cpuid_func: u32, iters: usize) -> ExecutableBuffer {
    emit_test_iters_rsi!(iters,
        ; mov   eax, cpuid_func as _
        ; cpuid
    )
}

