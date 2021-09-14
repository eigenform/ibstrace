
use std::io::BufRead;
use std::collections::BTreeMap;
use dynasmrt::AssemblyOffset;
use ibst::Sample;
use ibst::analysis::*;

/// Test a single MSR read, returning a list of unique memory accesses.
fn sample_msr(fd: i32, msr: u32, iters: usize) -> Box<[Sample]> {
    let code = ibst::codegen::emit_msr_test(msr, iters);
    let msg = ibst::ioctl::UserBuf::new(
        code.ptr(AssemblyOffset(0)), code.len()
    );
    ibst::measure(fd, &msg)
}

/// Test a list of MSRs, returning a map from ECX values to lists of unique 
/// memory accesses.
fn sample_msr_set(fd: i32, msr_list: &[u32]) -> BTreeMap<u32, Box<[Sample]>> {
    let mut map = BTreeMap::new();
    for msr in msr_list.iter() {
        eprintln!("sampling {:08x}", msr);
        let samples = sample_msr(fd, *msr, 0x10000);
        map.insert(*msr, samples);
    }
    map
}


fn main() -> Result<(), &'static str> {
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() < 2 {
        println!("usage: {} <filename>", arg[0]);
        return Err("Incorrect arguments");
    }
    let f = std::fs::File::open(&arg[1]).expect("Couldn't open file");

    let mut msr_list = Vec::new();
    for line in std::io::BufReader::new(f).lines() {
        if let Ok(msr) = line {
            msr_list.push(u32::from_str_radix(msr.as_str(), 16).unwrap());
        }
    }

    let fd = match ibst::ibstrace_open() {
        Ok(fd) => fd,
        Err(e) => return Err(e),
    };

    let per_msr_samples = sample_msr_set(fd, &msr_list);
    print_uniq_map_accesses(&per_msr_samples);

    ibst::ibstrace_close(fd);
    Ok(())
}
