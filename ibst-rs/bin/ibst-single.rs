use dynasmrt::{ 
    dynasm, DynasmApi, DynasmLabelApi,
    Assembler, AssemblyOffset, 
    x64::X64Relocation,
};
use ibst::emit_test_iters_rsi;
use ibst::analysis::*;

fn sample_instr(fd: i32, iters: usize) {
    let code = emit_test_iters_rsi!(iters, 
        ; rdrand rax
    );
    let msg = ibst::ioctl::UserBuf::new(code.ptr(AssemblyOffset(0)), code.len());
    let uniq_accesses = get_uniq_accesses(&ibst::measure(fd, &msg));
    for acc in &uniq_accesses {
        println!("{:?} {:016x} {:02}", acc.kind, acc.phys, acc.width);
    }
}

fn main() -> Result<(), &'static str> {
    let fd = match ibst::ibstrace_open() {
        Ok(fd) => fd,
        Err(e) => return Err(e),
    };
    sample_instr(fd, 0x1000);
    ibst::ibstrace_close(fd);
    Ok(())
}

