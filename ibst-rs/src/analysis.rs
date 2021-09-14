
use crate::*;
use std::collections::{ BTreeSet, BTreeMap };


/// Type of memory access (load or store).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryAccessKind { LD, ST }

/// A record of a sampled memory access.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAccess {
    /// The physical address tagged for this access.
    pub phys: usize,
    /// The width of this access in bytes.
    pub width: usize,
    /// The kind of access (load or store).
    pub kind: MemoryAccessKind,
}
impl MemoryAccess {
    pub fn from_sample(sample: &Sample) -> Option<Self> {
        let kind = match (sample.data3.st_op(), sample.data3.ld_op()) {
            (true, false) => MemoryAccessKind::ST,
            (false, true) => MemoryAccessKind::LD,
            (_, _) => return None,
        };
        let phys = sample.phyad;
        let width = sample.data3.op_mem_width() as usize;
        Some(Self { phys, width, kind })
    }
}

/// Return a list of the *unique* memory accesses in some set of samples.
pub fn get_uniq_accesses(samples: &[Sample]) -> BTreeSet<MemoryAccess> {
    let mut uniq_accesses = BTreeSet::<MemoryAccess>::new();
    for sample in samples.iter() {
        if sample.data.rip_invalid() { 
            continue; 
        }
        // We only care about microcoded instructions for now.
        if !sample.data.op_microcode() { 
            continue; 
        }
        let kind = match (sample.data3.st_op(), sample.data3.ld_op()) {
            (true, false) => MemoryAccessKind::ST,
            (false, true) => MemoryAccessKind::LD,
            (_, _) => continue,
        };
        let phys = sample.phyad;
        let width = sample.data3.op_mem_width() as usize;
        let tag_to_retire_cnt = sample.data.tag_to_ret_ctr();
        let completion_to_retire_cnt = sample.data.comp_to_ret_ctr();
        let access = MemoryAccess { phys, kind, width };
        uniq_accesses.insert(access);
    }
    uniq_accesses
}


pub fn print_uniq_map_accesses<K>(map: &BTreeMap<K, Box<[Sample]>>) where
    K: Clone + Copy + Ord + std::fmt::Debug + std::fmt::LowerHex
{
    let mut per_key_accs: BTreeMap<K, BTreeSet<MemoryAccess>> = BTreeMap::new();
    let mut unique_accs = BTreeMap::new();
    let mut common_accs = BTreeSet::new();

    for (key, samples) in map {
        per_key_accs.insert(*key, get_uniq_accesses(&samples));
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


