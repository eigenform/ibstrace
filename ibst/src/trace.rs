
use std::ops::RangeInclusive;

use crate::*;
use crate::codegen::*;
use crate::ibs::*;
use crate::analysis::run_precise_test;
use crate::ioctl::PreciseArgs;

use serde;
use serde::Serialize;

/// Properties of samples load/store ops. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize)]
pub struct LdstProps { 
    lin: usize,
    phy: usize,
    width: usize,
    locked: bool,
    uc: bool,
    wc: bool,
    swpf: bool,
    st: bool,
    ld: bool,
    src: NbDataSrc
}
impl LdstProps { 
    pub fn from_sample(s: &Sample) -> Self { 
        Self { 
            lin: s.linad,
            phy: s.phyad,
            ld: s.data3.ld_op(),
            st: s.data3.st_op(),
            width: s.data3.op_mem_width() as usize,
            locked: s.data3.dc_locked_op(),
            uc: s.data3.dc_uc_mem_acc(),
            wc: s.data3.dc_wc_mem_acc(),
            swpf: s.data3.sw_pf(),
            src: s.data2.data_src(),
        }
    }

    pub fn mnemonic(&self) -> &'static str { 
        match (self.ld, self.st) { 
            (false, false) => "??",
            (true, false)  => "LD",
            (false, true)  => "ST",
            (true, true)   => "LS",
        }
    }

    /// Return a string with the "properties" of this op.
    pub fn as_string(&self) -> String { 
        let locked = if self.locked { "[lock]" } else { "" };
        let uc = if self.uc { "[uc]" } else { "" };
        let wc = if self.wc { "[uc]" } else { "" };
        let swpf = if self.swpf { "[swpf]" } else { "" };
        let src = match self.src { 
            NbDataSrc::Dram => "[dram]",
            NbDataSrc::Invalid => "",
            NbDataSrc::Cache => "[cache]",
            NbDataSrc::Other => "[other]",
            NbDataSrc::Reserved1 => "[res1]",
            NbDataSrc::Reserved4 => "[res4]",
            NbDataSrc::Reserved5 => "[res5]",
            NbDataSrc::Reserved6 => "[res6]",
        };
        format!("{} {:3}b lin={:016x} phy={:016x} {}{}{}{}{}",
            self.mnemonic(),
            self.width, self.lin, self.phy,
            locked,uc,wc,swpf,src
        )
    }
}

/// Properties of sampled branch ops. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize)]
pub struct BrnProps {
    misp: bool,
    retired: bool,
    taken: bool,
    retrn: bool,
    fused: bool,
    tgt_rip: usize,
}
impl BrnProps {
    pub fn from_sample(s: &Sample) -> Self { 
        Self { 
            tgt_rip: s.tgt_rip,
            misp: s.data.op_brn_misp(),
            retired: s.data.op_brn_ret(),
            taken: s.data.op_brn_taken(),
            retrn: s.data.op_return(),
            fused: s.data.op_brn_fuse()
        }
    }

    pub fn mnemonic(&self) -> &'static str { 
        match (self.fused, self.retrn) { 
            (false, false) => "BR",
            (true, false)  => "BR",
            (false, true)  => "RT",
            (true, true)   => "RT",
        }
    }


    /// Return a string with the "properties" of this op.
    pub fn as_string(&self) -> String { 
        let miss = if self.misp { "[miss]" } else { "[hit]" };
        let ret = if self.retired { "[retire]" } else { "[specul]" };
        let dir = if self.taken { "[t]" } else { "[nt]" };
        let retrn = if self.retrn { "[return]" } else { "" };
        let fused = if self.retrn { "[fused]" } else { "" };
        format!("{} tgt={:016x} {}{}{}{}{}",
            self.mnemonic(), self.tgt_rip, miss, ret, dir, retrn, fused
        )
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize)]
pub struct TraceEntry {
    pub offset: usize,
    pub rip: usize,
    pub tag_to_retire: usize,
    pub complete_to_retire: usize,
    pub ucode: bool,
    pub ldst_props: Option<LdstProps>,
    pub brn_props: Option<BrnProps>,
}
impl TraceEntry { 
    pub fn from_sample(offset: usize, s: &Sample) -> Self { 

        let rip = s.rip;
        let tag_to_retire = s.data.tag_to_ret_ctr();
        let complete_to_retire = s.data.comp_to_ret_ctr();
        let ucode = s.data.op_microcode();

        let has_ldst = (
            (s.data3.st_op() || s.data3.ld_op())
        );

        let has_brn = (
            (s.data.op_brn_fuse() ||
            s.data.op_brn_ret() ||
            s.data.op_return() ||
            s.data.op_brn_misp() ||
            s.data.op_brn_taken())
        );


        let ldst_props = if has_ldst {
            Some(LdstProps::from_sample(s))
        } else { 
            None
        };

        let brn_props = if has_brn {
            Some(BrnProps::from_sample(s))
        } else { 
            None
        };


        Self {
            offset,
            rip,
            tag_to_retire,
            complete_to_retire,
            ucode,
            ldst_props,
            brn_props,
        }
    }
}


/// A collection of `ibstrace` samples corresponding to a "trace" of 
/// micro-ops in dispatched/retired order. 
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
pub struct Trace { 
    /// User-defined annotation
    pub annotation: String,

    /// Range of sampled offsets
    pub offset_range: RangeInclusive<usize>,

    /// Program counter of the instruction marked as the "target" in user code
    pub target_rip: usize,

    /// Container for `ibstrace` samples
    pub samples: Vec<TraceEntry>,
}
impl Trace { 

    /// Collect a trace 
    pub fn collect_from(
        params: &TestParameters,
        offset_range: RangeInclusive<usize>,
        rdi_val: usize,
    ) -> Result<Self, &'static str>
    {
        let mut samples = Vec::new();
        let base_addr = get_base_address()?;
        let target_rip = base_addr + params.tgt_instr_off;

        let fd = ibstrace_open()?;
        for offset in offset_range.clone() { 
            let res = run_precise_test(fd, &params, 
                PreciseArgs::new(rdi_val, offset)
            );
            if !res.is_empty() {
                let entry = TraceEntry::from_sample(offset, &res[0]);
                samples.push(entry);
            }
        }
        ibstrace_close(fd);

        Ok(Self { 
            samples,
            offset_range,
            target_rip,
            annotation: String::new(),
        })
    }

    pub fn annotate(&mut self, s: impl ToString) {
        self.annotation = s.to_string();
    }

    pub fn retain(&mut self, f: impl Fn(&TraceEntry) -> bool) {
        self.samples.retain(f);
    }

    pub fn to_json(&self) -> String { 
        use serde_json;
        serde_json::to_string(&self).unwrap()
    }

    pub fn print(&self) { 
        if !self.annotation.is_empty() {
            println!("[*] Trace '{}'", self.annotation);
        }
        println!("[*] Target RIP: {:016x}", self.target_rip);
        for entry in &self.samples { 

            // Tag-to-complete cycles
            let t2c = entry.tag_to_retire - entry.complete_to_retire;

            let lprops = if let Some(p) = entry.ldst_props {
                p.as_string()
            } else { 
                "".to_string()
            };
            let bprops = if let Some(p) = entry.brn_props {
                p.as_string()
            } else { 
                "".to_string()
            };

            println!("  {:08} {:016x} t2c={:05} {} {}",
                entry.offset,
                entry.rip,
                t2c, 
                lprops,
                bprops
            );
        }
    }
}


