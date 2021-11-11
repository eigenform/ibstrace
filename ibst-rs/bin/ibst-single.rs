use dynasmrt::{ 
    dynasm, DynasmApi, DynasmLabelApi,
    Assembler, AssemblyOffset, 
    x64::X64Relocation,
};
use ibst::emit_test_iters_rsi;
use ibst::codegen::*;
use ibst::analysis::*;

fn main() -> Result<(), &'static str> {

    let base_addr = ibst::get_base_address()?;
    let fd = ibst::ibstrace_open()?;

    let test = run_test(fd, emit_test_iters_rsi!(0x1000,
        ; ->target:
        ; rdrand rax
    ));

    let uniq_accesses = get_uniq_accesses(&test.result, base_addr);
    for acc in &uniq_accesses {
        println!("{:?} {:016x} {:02}", acc.kind, acc.phys, acc.width);
    }

    ibst::ibstrace_close(fd);
    Ok(())
}

