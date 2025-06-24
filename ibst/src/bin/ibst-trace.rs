
use dynasmrt::{ 
    dynasm, DynasmApi, DynasmLabelApi,
    Assembler, AssemblyOffset, 
    x64::X64Relocation,
    components::StaticLabel
};
use ibst::codegen::*;
use ibst::trace::*;
use std::fs::File;
use std::io::Write;

use ibst::Sample;
use std::collections::{HashMap, BTreeSet, BTreeMap};

/// Measured code (try sampling the RDTSC instruction)
fn emit_test() -> TestParameters {
    let mut asm = Assembler::<X64Relocation>::new().unwrap();
    dynasm!(asm
        ; ->target:
        ; rdtsc
        ; ->target_end:

        ; ret
    );

    let tgt_instr_off = match asm.labels()
        .resolve_static(&StaticLabel::global("target"))
    {
        Ok(offset) => offset.0,
        Err(e) => panic!("{:?}", e),
    };
    let tgt_instr_end = match asm.labels()
        .resolve_static(&StaticLabel::global("target_end"))
    {
        Ok(offset) => offset.0,
        Err(e) => panic!("{:?}", e),
    };
    let buf = asm.finalize().unwrap();
    TestParameters { buf, tgt_instr_off, tgt_instr_end }
}


fn main() -> Result<(), &'static str> {

    // Emit measured code
    let params = emit_test();

    // Collect a trace of some range of micro-ops
    let mut trace = Trace::collect_from(&params, 0..=128, 0)?;

    // Only keep ops associated with RDTSC
    let base_addr = ibst::get_base_address()?;
    let tgt_rip = base_addr + params.tgt_instr_off;
    trace.retain(|e| e.rip == tgt_rip);

    // Convenience method for easy output
    trace.print();

    // Serialize this trace to JSON and write to /tmp/
    let json = trace.to_json();
    let mut f = File::create("/tmp/ibs-trace.json").unwrap();
    f.write(json.as_bytes());

    Ok(())
}

