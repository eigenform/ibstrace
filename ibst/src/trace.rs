
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
    width: usize,
    locked: bool,
    uc: bool,
    wc: bool,
    swpf: bool,
    src: NbDataSrc
}
impl LdstProps { 
    pub fn from_sample(s: &Sample) -> Self { 
        Self { 
            width: s.data3.op_mem_width() as usize,
            locked: s.data3.dc_locked_op(),
            uc: s.data3.dc_uc_mem_acc(),
            wc: s.data3.dc_wc_mem_acc(),
            swpf: s.data3.sw_pf(),
            src: s.data2.data_src(),
        }
    }

    /// Return a string with the "properties" of this op.
    pub fn as_string(&self) -> String { 
        let mut res = String::new();
        res.push_str(&format!("[{:3}b]", self.width));
        if self.locked { res.push_str("[lock]"); }
        if self.uc { res.push_str("[uc]"); }
        if self.wc { res.push_str("[wc]"); }
        if self.swpf { res.push_str("[swpf]"); }
        match self.src { 
            NbDataSrc::Dram => res.push_str("[dram]"),
            NbDataSrc::Invalid => {},
            NbDataSrc::Cache => res.push_str("[cache]"),
            NbDataSrc::Other => res.push_str("[other]"),
            NbDataSrc::Reserved1 => res.push_str("[res1]"),
            NbDataSrc::Reserved4 => res.push_str("[res4]"),
            NbDataSrc::Reserved5 => res.push_str("[res5]"),
            NbDataSrc::Reserved6 => res.push_str("[res6]"),
        }
        res
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
}
impl BrnProps {
    pub fn from_sample(s: &Sample) -> Self { 
        Self { 
            misp: s.data.op_brn_misp(),
            retired: s.data.op_brn_ret(),
            taken: s.data.op_brn_taken(),
            retrn: s.data.op_return(),
            fused: s.data.op_brn_fuse()
        }
    }

    /// Return a string with the "properties" of this op.
    pub fn as_string(&self) -> String { 
        let mut res = String::new();
        let miss = if self.misp { "[miss]" } else { "[hit]" };
        let ret = if self.retired { "[retire]" } else { "[specul]" };
        let dir = if self.taken { "[t]" } else { "[nt]" };
        res.push_str(ret);
        res.push_str(dir);
        res.push_str(miss);
        if self.retrn { 
            res.push_str("[return]");
        }
        if self.fused { 
            res.push_str("[fused]");
        }
        res
    }
}

/// A micro-op corresponding to a [`Sample`] in some [`Trace`]. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize)]
pub enum TraceOp { 
    /// Load op
    Ld { lin: usize, phy: usize, prop: LdstProps },

    /// Store op
    St { lin: usize, phy: usize, prop: LdstProps },

    /// Load/Store op
    Ldst { lin: usize, phy: usize, prop: LdstProps },

    /// Branch op
    Brn { prop: BrnProps },

    /// Register op
    Reg { },

}
impl TraceOp { 
    /// Create a [`TraceOp`] from the given `ibstrace` [`Sample`].
    pub fn from_sample(s: &Sample) -> Self { 
        let is_st = s.data3.st_op();
        let is_ld = s.data3.ld_op();
        let is_brn = (
            s.data.op_brn_fuse() ||
            s.data.op_brn_ret() ||
            s.data.op_return() ||
            s.data.op_brn_misp() ||
            s.data.op_brn_taken()
        );

        match (is_ld, is_st) {
            (false, false) => {},
            (false, true) => {
                let prop = LdstProps::from_sample(s);
                return Self::St { lin: s.linad, phy: s.phyad, prop };
            },
            (true, false) => {
                let prop = LdstProps::from_sample(s);
                return Self::Ld { lin: s.linad, phy: s.phyad, prop };
            },
            (true, true) => {
                let prop = LdstProps::from_sample(s);
                return Self::Ldst { lin: s.linad, phy: s.phyad, prop };
            },
        }
        if is_brn { 
            let prop = BrnProps::from_sample(s);
            Self::Brn { prop }
        } else { 
            Self::Reg { }
        }
    }

    /// Return the "mnemonic" for this op.
    pub fn mnemonic(&self) -> &'static str { 
        match self { 
            Self::Ld{ .. }   => "LD",
            Self::St{ .. }   => "ST",
            Self::Ldst{ .. } => "LS",
            Self::Reg { }     => "RG",
            Self::Brn { .. }  => "BR",
        }
    }

    /// Return a string with the "properties" of this op.
    pub fn propstring(&self) -> String { 
        let mut res = String::new();
        match self { 
            Self::Reg {} => {},
            Self::Ld { lin, phy, prop } |
            Self::St { lin, phy, prop } |
            Self::Ldst { lin, phy, prop } => { 
                res = prop.as_string();
            },
            Self::Brn { prop } => {
                res = prop.as_string();
            },
        }
        res
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
    pub op: TraceOp,
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
                let entry = TraceEntry { 
                    offset: offset,
                    rip: res[0].rip,
                    tag_to_retire: res[0].data.tag_to_ret_ctr(),
                    complete_to_retire: res[0].data.comp_to_ret_ctr(),
                    ucode: res[0].data.op_microcode(),
                    op: TraceOp::from_sample(&res[0]),
                };
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

    pub fn to_json(&self) -> String { 
        use serde_json;
        serde_json::to_string(&self).unwrap()
    }

    pub fn print(&self) { 
        if !self.annotation.is_empty() {
            println!("[*] Trace '{}'", self.annotation);
        }
        println!("[*] Target RIP: {:016x}", self.target_rip);
        for sample in &self.samples { 

            // Tag-to-complete cycles
            let t2c = sample.tag_to_retire - sample.complete_to_retire;

            match sample.op {
                TraceOp::Ld { lin, phy, prop } |
                TraceOp::St { lin, phy, prop } |
                TraceOp::Ldst { lin, phy, prop } => { 
                    println!("  {:08} {:016x} {} t2c={:05} lin={:016x} phy={:016x} {}", 
                        sample.offset, 
                        sample.rip,
                        sample.op.mnemonic(), 
                        t2c, lin, phy, 
                        prop.as_string()
                    );
                },
                TraceOp::Reg {} => { 
                    println!("  {:08} {:016x} {} t2c={:05} {}", 
                        sample.offset, 
                        sample.rip,
                        sample.op.mnemonic(), 
                        t2c, 
                        ""
                    );
                },
                TraceOp::Brn{ prop } => {
                    println!("  {:08} {:016x} {} t2c={:05} {}", 
                        sample.offset, 
                        sample.rip,
                        sample.op.mnemonic(), 
                        t2c, 
                        prop.as_string()
                    );
                },
            }
        }
    }
}


