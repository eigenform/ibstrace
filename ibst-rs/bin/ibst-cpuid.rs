
use std::collections::{ BTreeMap, BTreeSet };
use dynasmrt::AssemblyOffset;
use ibst::Sample;
use ibst::analysis::*;

/// Test a single CPUID leaf, returning a list of samples
fn sample_cpuid_single(fd: i32, eax: u32, iters: usize) -> Box<[Sample]> {
    let code = ibst::codegen::emit_cpuid_test(eax, iters);
    let msg = ibst::ioctl::UserBuf::new(
        code.ptr(AssemblyOffset(0)), code.len()
    );
    ibst::measure(fd, &msg)
}

/// Test all valid/typical CPUID leaves, returning a map from leaf numbers to
/// lists of samples.
fn sample_cpuid_known(fd: i32) -> BTreeMap<u32, Box<[Sample]>> {
    let mut map: BTreeMap<u32, Box<[Sample]>> = BTreeMap::new();
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
    let fd = match ibst::ibstrace_open() {
        Ok(fd) => fd,
        Err(e) => return Err(e),
    };

    let per_leaf_samples = sample_cpuid_known(fd);
    print_uniq_map_accesses(&per_leaf_samples);

    ibst::ibstrace_close(fd);
    Ok(())
}


