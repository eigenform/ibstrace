//! One-off tests.
//!
//! ## Observations
//!
//!

use dynasmrt::{ 
    dynasm, DynasmApi, DynasmLabelApi,
    Assembler, AssemblyOffset, 
    x64::X64Relocation,
};
use ibst::emit_test_iters_rsi;
use ibst::codegen::*;
use ibst::analysis::*;

use ibst::Sample;
use std::collections::{HashMap, BTreeSet, BTreeMap};

const RDPRU: [u8; 3] = [ 0x0f, 0x01, 0xfd ];
const WBNOINVD: [u8; 3] = [ 0xf3, 0x0f, 0x09];
const RDSSPQ_RAX: [u8; 5] = [ 0xf3, 0x48, 0x0f, 0x1e, 0xc8 ];

fn gentest(val: u32, iters: usize) -> TestParameters {
    emit_test_iters_rsi!(iters,
        ; xor eax, eax
        ; xor edx, edx
        ; mov ecx, val as _
        ; ->target:
        ; rdpmc
    )
}



fn main() -> Result<(), &'static str> {

    let base_addr = ibst::get_base_address()?;
    let fd = ibst::ibstrace_open()?;

    let params = gentest(0, 0x400000);
    let rip = base_addr + params.tgt_instr_off;
    let res = run_test(fd, params);
    println!("[*] Collected {} samples", res.result.len());
    print_load_lat_dist(&res.result, rip);

    //print_samples(&res.result, rip);
    //print_uniq_samples(&res.result, rip);

    ibst::ibstrace_close(fd);
    Ok(())
}

