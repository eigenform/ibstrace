
use crate::*;
use crate::ibs::*;
use crate::codegen::TestParameters;
use std::collections::{ BTreeSet, BTreeMap };
use dynasmrt::{ AssemblyOffset, ExecutableBuffer };

/// Some type of memory access (either a load, or a store).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryAccessKind { LD, ST, LDST }

/// A record of a sampled memory access.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAccess {
    /// The physical address tagged for this access.
    pub phys: usize,

    /// The width of this access in bytes.
    pub width: usize,

    /// The kind of access (load or store).
    pub kind: MemoryAccessKind,
}
impl MemoryAccess {
    /// Try to obtain a [MemoryAccess] from a [Sample].
    pub fn from_sample(sample: &Sample) -> Option<Self> {
        let kind = match (sample.data3.st_op(), sample.data3.ld_op()) {
            (true, false) => MemoryAccessKind::ST,
            (false, true) => MemoryAccessKind::LD,
            (true, true) => MemoryAccessKind::LDST,
            (_, _) => return None,
        };
        let phys = sample.phyad;
        let width = sample.data3.op_mem_width() as usize;
        Some(Self { phys, width, kind })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct SampleInfo {
    pub data:  ibs::IbsOpData, 
    pub data2: ibs::IbsOpData2, 
    pub data3: ibs::IbsOpData3, 
    pub linad: usize, 
    pub phyad: usize,
    pub tgt_rip: usize,
}
impl SampleInfo {
    pub fn from_sample(s: &Sample) -> Self { 
        Self { 
            data: s.data,
            data2: s.data2,
            data3: s.data3,
            linad: s.linad,
            phyad: s.phyad,
            tgt_rip: s.tgt_rip,
        }
    }
}
impl std::fmt::Display for SampleInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, 
            "[t2r={:05} c2r={:05} uc={} brn_ret={} brn_tkn={} brn_msp={} phy={:016x} tgt={:016x}",
            self.data.tag_to_ret_ctr(),
            self.data.comp_to_ret_ctr(),
            self.data.op_microcode() as usize,
            self.data.op_brn_ret() as usize,
            self.data.op_brn_taken() as usize,
            self.data.op_brn_misp() as usize,
            self.phyad,
            self.tgt_rip,
        )
    }
}


/// The results of a particular test.
pub struct TestResult {
    /// Information about the code emitted for this test.
    pub params: TestParameters,
    /// The resulting samples from this test.
    pub result: Box<[Sample]>,
}

/// Given some [TestParameters], submit a test and return the results.
pub fn run_test(fd: i32, params: TestParameters) -> TestResult {
    let msg = params.to_userbuf();
    let result = crate::measure(fd, &msg);
    TestResult { params, result }
}

pub fn run_precise_test(
    fd: i32, 
    params: &TestParameters,
    arg: ioctl::PreciseArgs,
) 
    -> Box<[Sample]>
{
    let msg = params.to_userbuf();
    let result = crate::measure_precise(fd, &msg, &arg);
    result
}




pub fn filter_by_rip(samples: &[Sample], tgt_rip: usize) 
    -> impl Iterator<Item = &Sample> 
{
    samples.iter().filter(move |&x| x.rip == tgt_rip)
}

pub fn print_sample(s: &Sample) {
    //println!("[*] IbsOpCtl:  {:016x}", s.ctl.0);
    println!("[*] IbsOpRip:   {:016x} (valid={})", 
             s.rip, !s.data.rip_invalid());
    println!("[*] IbsOpData:  {:016x}", s.data.0);
    //println!("  reserved bits (hi):         {:x}", s.data.res_hi());
    //println!("  reserved bits (lo):         {:x}", s.data.res_lo());
    println!("  tag-to-retire count:        {}", s.data.tag_to_ret_ctr());
    println!("  completion-to-retire count: {}", s.data.comp_to_ret_ctr());
    if s.data.op_microcode() { println!("  microcode op") }
    if s.data.op_brn_fuse() {  println!("  fused branch"); }
    if s.data.rip_invalid() {  println!("  RIP invalid"); }
    if s.data.op_brn_ret() {   println!("  retired branch"); }
    if s.data.op_brn_misp() {  println!("  mispredicted branch"); }
    if s.data.op_brn_taken() { println!("  taken branch"); }
    if s.data.op_return() {    println!("  return op"); }
    if s.data.res_33() {       println!("  reserved bit 33?"); }
    if s.data.res_32() {       println!("  reserved bit 32?"); }

    println!("[*] IbsOpData2: {:016x}", s.data2.0);
    if s.data2.data_src() != NbDataSrc::Invalid {
        println!("  Data source: {:?}", s.data2.data_src());
        if s.data2.data_src() == NbDataSrc::Cache {
            println!("  Cache hit state: {}", s.data2.cache_hit_st());
        }
    }
    println!("[*] IbsOpData3: {:016x}", s.data3.0);
    if s.data3.0 != 0 {
        //println!("  res_lo:  {:04b}", s.data3.res_lo());
        println!("  PhyAddr: {:016x} (valid={})", 
            s.phyad, s.data3.dc_phy_addr_valid());
        println!("  LinAddr: {:016x} (valid={})", 
            s.linad, s.data3.dc_lin_addr_valid());
        println!("  Width:   {:?}", s.data3.op_mem_width());
        if s.data3.st_op() { println!("  Store op"); }
        if s.data3.ld_op() { println!("  Load op"); }

        if s.data3.sw_pf() { 
            println!("  Software prefetch op"); 
        }
        if s.data3.dc_l2_miss() { 
            println!("  L2 cache miss"); 
        }
        if s.data3.dc_miss() { 
            println!("  Data cache miss");
        }
        if s.data3.dc_mis_acc() {
            println!("  Misaligned access (crossing cache line)");
        }
        if s.data3.dc_locked_op() {
            println!("  Locked op");
        }
        if s.data3.dc_uc_mem_acc() {
            println!("  Uncacheable access");
        }
        if s.data3.dc_uc_mem_acc() {
            println!("  Write-combining access");
        }
    }

    if s.tgt_rip != 0 {
    //if s.data.op_brn_ret() {
        println!("[*] IbsTgtRip:  {:016x}", s.tgt_rip);  
    }

    println!("");
}


pub fn print_samples(samples: &[Sample], tgt_rip: usize) {
    for (idx, s) in filter_by_rip(&samples, tgt_rip).enumerate() {
        print_sample(&s);
    }
}

/// Print the latency distribution for all unique load operations.
pub fn print_load_lat_dist(samples: &[Sample], tgt_rip: usize) {
    use std::collections::btree_map::{ BTreeMap, Entry };

    struct Stats { 
        pub tag2ret: Vec<usize>, 
        pub comp2ret: Vec<usize> 
    }
    impl Stats {
        fn new_from_sample(s: &Sample) -> Self { 
            let mut ret = Self { tag2ret: Vec::new(), comp2ret: Vec::new() };
            ret.tag2ret.push(s.data.tag_to_ret_ctr());
            ret.comp2ret.push(s.data.comp_to_ret_ctr());
            ret
        }
    }

    let mut stats: BTreeMap<MemoryAccess, Stats> = BTreeMap::new();
    for s in filter_by_rip(&samples, tgt_rip).filter(|x| x.data3.ld_op()) { 
        if let Some(acc) = MemoryAccess::from_sample(s) {
            match stats.entry(acc) {
                Entry::Vacant(map) => { 
                    map.insert(Stats::new_from_sample(s));
                },
                Entry::Occupied(mut map) => {
                    map.get_mut().tag2ret.push(s.data.tag_to_ret_ctr());
                    map.get_mut().comp2ret.push(s.data.comp_to_ret_ctr());
                }
            }
        }
    }

    println!("[*] Sample distribution:");
    for (acc, stats) in stats {
        println!("{:016x} {:02} {:?} ({} samples)", 
                 acc.phys, acc.width, acc.kind, stats.tag2ret.len());
        println!("  tag2ret  min={} max={}", 
                 stats.tag2ret.iter().min().unwrap_or(&0), 
                 stats.tag2ret.iter().max().unwrap_or(&0),
        );
        println!("  comp2ret min={} max={}", 
                 stats.comp2ret.iter().min().unwrap_or(&0), 
                 stats.comp2ret.iter().max().unwrap_or(&0),
        );

    }
    println!("");
}

pub fn print_uniq_samples(samples: &[Sample], tgt_rip: usize) {
    use std::collections::HashSet;
    let mut uniq = HashSet::new();
    for s in filter_by_rip(&samples, tgt_rip) { 
        uniq.insert(s); 
    }
    for (idx, s) in uniq.iter().enumerate() { 
        println!("[*] Unique sample {}", idx);
        print_sample(&s); 
        println!("");
    }
}

/// Return a list of the *unique* memory accesses in some set of samples.
pub fn get_uniq_accesses(samples: &[Sample], tgt_rip: usize) 
    -> BTreeSet<MemoryAccess>
{
    let mut uniq_accesses = BTreeSet::<MemoryAccess>::new();

    for sample in filter_by_rip(&samples, tgt_rip) {
        if sample.data.rip_invalid() { 
            continue; 
        }
        let kind = match (sample.data3.st_op(), sample.data3.ld_op()) {
            (true, false) => MemoryAccessKind::ST,
            (false, true) => MemoryAccessKind::LD,
            (true, true) => MemoryAccessKind::LDST,
            (_, _) => continue,
        };
        let phys = sample.phyad;
        let width = sample.data3.op_mem_width() as usize;
        let access = MemoryAccess { phys, kind, width };
        uniq_accesses.insert(access);
    }
    uniq_accesses
}


pub fn print_uniq_map_accesses<K>(map: &BTreeMap<K, TestResult>, buf: usize)
    where K: Clone + Copy + Ord + std::fmt::Debug + std::fmt::LowerHex
{
    let mut per_key_accs: BTreeMap<K, BTreeSet<MemoryAccess>> = BTreeMap::new();
    let mut unique_accs = BTreeMap::new();
    let mut common_accs = BTreeSet::new();

    for (key, test) in map {
        let tgt_rip = buf + test.params.tgt_instr_off;
        per_key_accs.insert(*key, get_uniq_accesses(&test.result, tgt_rip));
    }
    for (cur_key, accs) in &per_key_accs {
        let mut uniq = Vec::new();
        for acc in accs.iter() {
            if per_key_accs.iter()
                        .all(|(_, a)| { a.contains(acc) }) {
                common_accs.insert(acc);
            }
            if !per_key_accs.iter().filter(|(k, _)| { cur_key != *k })
                            .any(|(_, a)| { a.contains(acc) }) {
                uniq.push(acc);
            }
        }
        if !uniq.is_empty() {
            unique_accs.insert(cur_key, uniq);
        }
    }

    println!("Common accesses (among all keys):");
    for acc in common_accs.iter() {
        println!("{:016x} {:02} {:?}", acc.phys, acc.width, acc.kind);
    }
    println!("");

    for (key, accs) in unique_accs {
        println!("Unique accesses for key {:08x?}:", key);
        for acc in accs.iter() {
            println!("  {:016x} {:02} {:?}", acc.phys, acc.width, acc.kind);
        }
        println!("");
    }

    for (cur_key, accs) in &per_key_accs {
        println!("All accesses for key {:08x?}", cur_key);
        for acc in accs.iter() {
            println!("  {:016x} {:02} {:?}", acc.phys, acc.width, acc.kind);
        }
        println!("");
    }
}

