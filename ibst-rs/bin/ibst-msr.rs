
use std::io::BufRead;
use std::collections::*;
use dynasmrt::AssemblyOffset;
use ibst::Sample;
use ibst::analysis::*;
use itertools::*;
use ibst::msr::*;
use num_enum::*;
use clap::Parser;

/// ibst-msr
/// ========
///
/// Use the 'ibstrace' kernel module to analyze the memory accesses produced 
/// by 'rdmsr' for different MSRs. 
///
/// WARNING: 'ibstrace' does not validate input to 'rdmsr' and does not 
/// gracefully handle any exceptions/faults produced by 'rdmsr'. 
///
#[derive(Parser)]
#[command(verbatim_doc_comment)]
struct Args { 
    /// Add a list of MSRs to test from a file (one-per-line)
    #[arg(long,)]
    list_file: Option<String>,

    /// Add one or more MSRs to test, ie. '--msr=c0000080,000000e7,
    #[arg(long,value_delimiter=',',num_args=1..,)]
    msr: Option<Vec<String>>,

}

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
        let samples = sample_msr(fd, *msr, 0x8_0000);
        map.insert(*msr, samples);
    }
    map
}

pub fn print_results(map: &BTreeMap<u32, TestResult>, buf: usize)
{
    let num_msrs = map.len();

    // Associate each MSR to the set of observed memory accesses
    let mut per_msr_accs: BTreeMap<u32, BTreeSet<MemoryAccess>> = BTreeMap::new();
    for (msr, test) in map {
        let tgt_rip = buf + test.params.tgt_instr_off;
        per_msr_accs.insert(*msr, get_uniq_accesses(&test.result, tgt_rip));
    }

    // Associate each access to a set of MSRs 
    let mut accs_by_msr: BTreeMap<MemoryAccess, BTreeSet<u32>> = BTreeMap::new();
    for (msr, accs) in &per_msr_accs {
        for acc in accs.iter() { 
            if let Some(msrs) = accs_by_msr.get_mut(acc) {
                msrs.insert(*msr);
            } else { 
                let mut msrs = BTreeSet::new();
                msrs.insert(*msr);
                accs_by_msr.insert(*acc, msrs);
            }
        }
    }

    // Set of accesses that are unique to a particular MSR
    let uniq_accs = accs_by_msr.iter().filter(|(acc, msrs)| { 
        msrs.len() == 1 
    });

    // Set of accesses that occurred for all MSRs
    let common_accs = accs_by_msr.iter().filter(|(acc, msrs)| { 
        msrs.len() == num_msrs
    });

    println!("[*] Accesses unique to a single MSR:");
    for (acc, msrs) in uniq_accs { 
        let msr_num = *msrs.first().unwrap();
        let msr_name = if let Ok(msr) = Msr::try_from_primitive(msr_num) {
            format!("{:?}", msr)
        } else { 
            format!("unknown")
        };
        println!("  {:016x} {:02x} {:4?} => {:08x} ({})",  
            acc.phys, acc.width, acc.kind, msr_num, msr_name
        );

    }


    //let mut unique_accs = BTreeMap::new();
    //let mut common_accs = BTreeSet::new();

    //for (msr, accs) in &per_msr_accs {
    //    let mut uniq = Vec::new();
    //    for acc in accs.iter() {
    //        if per_msr_accs.iter().all(|(_, a)| { a.contains(acc) }) {
    //            common_accs.insert(acc);
    //        }
    //        if !per_msr_accs.iter()
    //            .filter(|(k, _)| { msr != *k })
    //            .any(|(_, a)| { a.contains(acc) }) 
    //        {
    //            uniq.push(acc);
    //        }
    //    }
    //    if !uniq.is_empty() {
    //        unique_accs.insert(msr, uniq);
    //    }
    //}

    //println!("Common accesses (among all keys):");
    //for acc in common_accs.iter() {
    //    println!("{:016x} {:02} {:?}", acc.phys, acc.width, acc.kind);
    //}
    //println!("");

    //for (msr, accs) in unique_accs {
    //    println!("Unique accesses for MSR {:08x?}:", msr);
    //    for acc in accs.iter() {
    //        println!("  {:016x} {:02} {:?}", acc.phys, acc.width, acc.kind);
    //    }
    //    println!("");
    //}

    //for (msr, accs) in &per_msr_accs {
    //    println!("All accesses for MSR {:08x?}", msr);
    //    for acc in accs.iter() {
    //        println!("  {:016x} {:02} {:?}", acc.phys, acc.width, acc.kind);
    //    }
    //    println!("");
    //}
}



fn main() -> Result<(), &'static str> {
    let arg = Args::parse();

    let mut msr_set: BTreeSet<u32> = BTreeSet::new();

    if let Some(filename) = arg.list_file {
        println!("[*] Appending MSRs from file '{}'", filename);
        let f = std::fs::File::open(filename).expect("Couldn't open file");
        for line in std::io::BufReader::new(f).lines() {
            if let Ok(msr) = line {
                let msr_num = u32::from_str_radix(msr.as_str(), 16)
                    .expect("error parsing input from file");
                msr_set.insert(msr_num);
            }
        }
    }

    if let Some(list) = arg.msr {
        println!("{:?}", list);
        for x in list { 
            let msr_num = u32::from_str_radix(x.as_str(), 16)
                .expect("error while parsing --msr");
            msr_set.insert(msr_num);
        }
    }

    let msr_list: Vec<u32> = msr_set.iter().map(|e| *e).collect();

    let base_addr = ibst::get_base_address()?;
    let fd = ibst::ibstrace_open()?;

    let per_msr_samples = sample_msr_set(fd, &msr_list);
    print_results(&per_msr_samples, base_addr);

    ibst::ibstrace_close(fd);
    Ok(())
}


