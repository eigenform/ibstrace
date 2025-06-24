//! One-off tests.

use dynasmrt::{ 
    dynasm, DynasmApi, DynasmLabelApi,
    Assembler, AssemblyOffset, 
    x64::X64Relocation,
    components::StaticLabel
};
use ibst::emit_test_iters_rsi;
use ibst::codegen::*;
use ibst::analysis::*;

use ibst::Sample;
use std::collections::{HashMap, BTreeSet, BTreeMap};

/// Emit some code we want to measure
fn emit_test(val: u32, iters: usize) -> TestParameters {
    emit_test_iters_rsi!(iters,
        ; xor rax, rax
        ; xor rdx, rdx
        ; ->target:
        ; rdtsc
    )
}


fn main() -> Result<(), &'static str> {
    let base_addr = ibst::get_base_address()?;
    let fd = ibst::ibstrace_open()?;

    // Emit measured code
    let params = emit_test(0, 0x100000);

    // Get the range of program counter values associated with measured code.
    // In this case, we're expecting IBS samples from a single instruction 
    // (located at `target_start`). 
    let target_start = base_addr + params.tgt_instr_off;
    let target_end   = base_addr + params.tgt_instr_end;
    println!("[*] Base addr:    {:016x}", base_addr);
    println!("[*] target_start: {:016x}", target_start);
    println!("[*] target_end:   {:016x}", target_end);

    // Upload measured code and use the "measure" ioctl() to collect samples
    let res = run_test(fd, params);
    println!("[*] Collected {} samples", res.result.len());

    // Print sampled load/store ops for RDTSC
    print_load_lat_dist(&res.result, target_start);

    let mut by_pc: BTreeMap<usize, BTreeSet<SampleInfo>> = BTreeMap::new();
    for sample in res.result.iter() {
        if let Some(mut set) = by_pc.get_mut(&sample.rip) {
            set.insert(SampleInfo::from_sample(sample));
        } else { 
            let mut set = BTreeSet::new();
            set.insert(SampleInfo::from_sample(sample));
            by_pc.insert(sample.rip, set);
        }
    }

    ibst::ibstrace_close(fd);
    Ok(())
}

