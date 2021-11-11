
use std::io::BufRead;
use std::collections::BTreeMap;
use dynasmrt::AssemblyOffset;
use ibst::Sample;
use ibst::analysis::*;

/// Test a single MSR read, returning a list of unique memory accesses.
fn sample_msr(fd: i32, msr: u32, iters: usize) -> TestResult {
    run_test(fd, ibst::codegen::emit_msr_test(msr, iters))
}

/// Test a list of MSRs, returning a map from ECX values to lists of unique 
/// memory accesses.
fn sample_msr_set(fd: i32, msr_list: &[u32]) -> BTreeMap<u32, TestResult> {
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

    let base_addr = ibst::get_base_address()?;
    let fd = ibst::ibstrace_open()?;

    let per_msr_samples = sample_msr_set(fd, &msr_list);
    print_uniq_map_accesses(&per_msr_samples, base_addr);

    ibst::ibstrace_close(fd);
    Ok(())
}
