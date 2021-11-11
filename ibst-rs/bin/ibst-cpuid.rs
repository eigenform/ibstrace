
use std::collections::{ BTreeMap, BTreeSet };
use dynasmrt::AssemblyOffset;
use ibst::codegen::*;
use ibst::analysis::*;
use ibst::Sample;

/// Test a single CPUID leaf, returning a list of samples
fn sample_cpuid_single(fd: i32, eax: u32, iters: usize) -> TestResult {
    run_test(fd, ibst::codegen::emit_cpuid_test(eax, iters))
}

/// Test all valid/typical CPUID leaves, returning a map from leaf numbers to
/// lists of samples.
fn sample_cpuid_known(fd: i32) -> BTreeMap<u32, TestResult> {
    let mut map: BTreeMap<u32, TestResult> = BTreeMap::new();
    for eax in 0x0000_0000..=0x0000_0020 {
        let samples = sample_cpuid_single(fd, eax, 0x100000);
        map.insert(eax, samples);
    }
    for eax in 0x8000_0000..=0x8000_0020 {
        let samples = sample_cpuid_single(fd, eax, 0x100000);
        map.insert(eax, samples);
    }
    map
}


fn main() -> Result<(), &'static str> {
    let base_addr = ibst::get_base_address()?;
    let fd = ibst::ibstrace_open()?;

    let per_leaf_samples = sample_cpuid_known(fd);
    print_uniq_map_accesses(&per_leaf_samples, base_addr);

    ibst::ibstrace_close(fd);
    Ok(())
}


