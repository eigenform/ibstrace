//! Related MSR definitions for AMD IBS.
//!
//! See "PPR for AMD Family 17h Model 71h B0".
//!
//! NOTE: You're deriving `Hash` for all of these, but there are some fields
//! that you might not want to hash when distinguishing between unique 
//! samples.

use std::hash::{Hash, Hasher};

/// MSRC001_1033 [IBS Execution Control] (Core::X86::Msr::IBS_OP_CTL)
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct IbsOpCtl(pub usize);
impl IbsOpCtl {
    const RES_63_59_MASK:   usize = 0xf100_0000_0000_0000;
    const CUR_CNT_MASK:     usize = 0x07ff_ffff_0000_0000;
    const RES_31_27_MASK:   usize = 0x0000_0000_f100_0000;

    pub fn cur_cnt(&self) -> usize { 
        (self.0 & Self::CUR_CNT_MASK) >> 32 
    }
}

/// MSRC001_1035 [IBS Op Data] (Core::X86::Msr::IBS_OP_DATA)
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IbsOpData(pub usize);
impl IbsOpData {
    const RES_63_41_MASK:       usize = 0xffff_fe00_0000_0000;
    const OP_MICROCODE_BIT:     usize = 0x0000_0100_0000_0000;
    const OP_BRN_FUSE_BIT:      usize = 0x0000_0080_0000_0000;
    const RIP_INVALID_BIT:      usize = 0x0000_0040_0000_0000;
    const OP_BRN_RET_BIT:       usize = 0x0000_0020_0000_0000;
    const OP_BRN_MISP_BIT:      usize = 0x0000_0010_0000_0000;
    const OP_BRN_TAKEN_BIT:     usize = 0x0000_0008_0000_0000;
    const OP_RETURN_BIT:        usize = 0x0000_0004_0000_0000;

    const RES_33:               usize = 0x0000_0002_0000_0000;
    const RES_32:               usize = 0x0000_0001_0000_0000;
    const RES_33_32_MASK:       usize = 0x0000_0003_0000_0000;

    const TAG_TO_REG_CTR_MASK:  usize = 0x0000_0000_ffff_0000;
    const COMP_TO_RET_CTR_MASK: usize = 0x0000_0000_0000_ffff;

    pub fn res_hi(&self) -> usize { (self.0 & Self::RES_63_41_MASK) >> 41 }
    pub fn res_lo(&self) -> usize { (self.0 & Self::RES_33_32_MASK) >> 32 }

    pub fn res_33(&self) -> bool { 
        (self.0 & Self::RES_33) != 0 
    }
    pub fn res_32(&self) -> bool { 
        (self.0 & Self::RES_32) != 0 
    }



    pub fn op_microcode(&self) -> bool { 
        (self.0 & Self::OP_MICROCODE_BIT) != 0 
    }
    pub fn op_brn_fuse(&self) -> bool { 
        (self.0 & Self::OP_BRN_FUSE_BIT) != 0 
    }
    pub fn rip_invalid(&self) -> bool { 
        (self.0 & Self::RIP_INVALID_BIT) != 0 
    }
    pub fn op_brn_ret(&self) -> bool { 
        (self.0 & Self::OP_BRN_RET_BIT) != 0 
    }
    pub fn op_brn_misp(&self) -> bool { 
        (self.0 & Self::OP_BRN_MISP_BIT) != 0 
    }
    pub fn op_brn_taken(&self) -> bool { 
        (self.0 & Self::OP_BRN_TAKEN_BIT) != 0 
    }
    pub fn op_return(&self) -> bool { 
        (self.0 & Self::OP_RETURN_BIT) != 0 
    }
    pub fn tag_to_ret_ctr(&self) -> usize { 
        (self.0 & Self::TAG_TO_REG_CTR_MASK) >> 16 
    }
    pub fn comp_to_ret_ctr(&self) -> usize { 
        self.0 & Self::COMP_TO_RET_CTR_MASK 
    }
}

impl std::fmt::Debug for IbsOpData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        fmt.debug_struct("IbsOpData")
            .field("ucode", &self.op_microcode())
            .field("fused_brn", &self.op_brn_fuse())
            .field("op_brn_ret", &self.op_brn_ret())
            .field("tag_to_retire", &self.tag_to_ret_ctr())
            .field("complete_to_retire", &self.comp_to_ret_ctr())
            .field("res_33", &self.res_33())
            .field("res_32", &self.res_32())
            .finish()
    }
}


#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum NbDataSrc {
    Invalid,
    Reserved1,
    Cache,
    Dram,
    Reserved4,
    Reserved5,
    Reserved6,
    Other,
}
impl From<usize> for NbDataSrc{  
    fn from(x: usize) -> Self {
        match x {
            0 => Self::Invalid,
            1 => Self::Reserved1,
            2 => Self::Cache,
            3 => Self::Dram,
            4 => Self::Reserved4,
            5 => Self::Reserved5,
            6 => Self::Reserved6,
            7 => Self::Other,
            _ => unimplemented!(),
        }
    }
}

/// MSRC001_1036 [IBS Op Data 2] (Core::X86::Msr::IBS_OP_DATA2)
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IbsOpData2(pub usize);
impl IbsOpData2 {
    const CACHE_HIT_ST_BIT:     usize = (1 << 5);
    const RMT_NODE_BIT:         usize = (1 << 4);
    const DATA_SRC_MASK:        usize = 0x0000_0000_0000_0007;
    pub fn cache_hit_st(&self) -> bool {
        (self.0 & Self::CACHE_HIT_ST_BIT) != 0 
    }
    pub fn rmt_node(&self) -> bool {
        (self.0 & Self::RMT_NODE_BIT) != 0 
    }
    pub fn data_src(&self) -> NbDataSrc {
        NbDataSrc::from(self.0 & Self::DATA_SRC_MASK)
    }
}

/// MSRC001_1037 [IBS Op Data 3] (Core::X86::Msr::IBS_OP_DATA3)
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IbsOpData3(pub usize);
impl IbsOpData3 {
    const TLB_REFILL_LAT_MASK:  usize = 0xffff_0000_0000_0000;
    const DC_MISS_LAT_MASK:     usize = 0x0000_ffff_0000_0000;

    const OP_DC_MISS_OPEN_MEM_REQS_MASK: 
                                usize = 0x0000_0000_fc00_0000;

    const OP_MEM_WIDTH_MASK:    usize = 0x0000_0000_03c0_0000;

    const SW_PF_BIT:            usize = 0x0000_0000_0020_0000;
    const DC_L2_MISS_BIT:       usize = 0x0000_0000_0010_0000;
    const DC_L2TLB_HIT_1G_BIT:  usize = 0x0000_0000_0008_0000;
    const DC_PHY_ADDR_VAL_BIT:  usize = 0x0000_0000_0004_0000;
    const DC_LIN_ADDR_VAL_BIT:  usize = 0x0000_0000_0002_0000;

    const DC_MISS_NO_MAB_ALLOC_BIT: 
                                usize = 0x0000_0000_0001_0000;

    const DC_LOCKED_OP_BIT:     usize = 0x0000_0000_0000_8000;
    const DC_UC_MEM_ACC_BIT:    usize = 0x0000_0000_0000_4000;
    const DC_WC_MEM_ACC_BIT:    usize = 0x0000_0000_0000_2000;

    const RES_12_9_MASK:        usize = 0x0000_0000_0000_1e00;

    const DC_MIS_ACC_BIT:       usize = 0x0000_0000_0000_0100;
    const DC_MISS_BIT:          usize = 0x0000_0000_0000_0080;
    const DC_L2TLB_HIT_2M_BIT:  usize = 0x0000_0000_0000_0040;
    const DC_L1TLB_HIT_1G_BIT:  usize = 0x0000_0000_0000_0020;
    const DC_L1TLB_HIT_2M_BIT:  usize = 0x0000_0000_0000_0010;
    const DC_L2TLB_MISS_BIT:    usize = 0x0000_0000_0000_0008;
    const DC_L1TLB_MISS_BIT:    usize = 0x0000_0000_0000_0004;
    const ST_OP_BIT:            usize = 0x0000_0000_0000_0002;
    const LD_OP_BIT:            usize = 0x0000_0000_0000_0001;

    pub fn res_lo(&self) -> usize {
        (self.0 & Self::RES_12_9_MASK) >> 9
    }

    pub fn tlb_refill_lat(&self) -> usize { 
        (self.0 & Self::TLB_REFILL_LAT_MASK) >> 48 
    }
    pub fn dc_miss_lat(&self) -> usize { 
        (self.0 & Self::DC_MISS_LAT_MASK) >> 32 
    }
    pub fn op_dc_miss_open_mem_reqs(&self) -> usize { 
        (self.0 & Self::OP_DC_MISS_OPEN_MEM_REQS_MASK) >> 26 
    }
    pub fn op_mem_width(&self) -> IbsMemWidth {
        IbsMemWidth::from((self.0 & Self::OP_MEM_WIDTH_MASK) >> 22)
    }

    pub fn sw_pf(&self) -> bool { 
        (self.0 & Self::SW_PF_BIT) != 0 
    }
    pub fn dc_l2_miss(&self) -> bool { 
        (self.0 & Self::DC_L2_MISS_BIT) != 0 
    }
    pub fn dc_l2tlb_hit_1g(&self) -> bool { 
        (self.0 & Self::DC_L2TLB_HIT_1G_BIT) != 0 
    }

    pub fn dc_phy_addr_valid(&self) -> bool { 
        (self.0 & Self::DC_PHY_ADDR_VAL_BIT) != 0 
    }
    pub fn dc_lin_addr_valid(&self) -> bool { 
        (self.0 & Self::DC_LIN_ADDR_VAL_BIT) != 0 
    }
    pub fn dc_miss_no_mab_alloc(&self) -> bool { 
        (self.0 & Self::DC_MISS_NO_MAB_ALLOC_BIT) != 0 
    }

    pub fn dc_locked_op(&self) -> bool { 
        (self.0 & Self::DC_LOCKED_OP_BIT) != 0 
    }
    pub fn dc_uc_mem_acc(&self) -> bool { 
        (self.0 & Self::DC_UC_MEM_ACC_BIT) != 0 
    }
    pub fn dc_wc_mem_acc(&self) -> bool { 
        (self.0 & Self::DC_WC_MEM_ACC_BIT) != 0 
    }

    pub fn dc_mis_acc(&self) -> bool { 
        (self.0 & Self::DC_MIS_ACC_BIT) != 0 
    }
    pub fn dc_miss(&self) -> bool { 
        (self.0 & Self::DC_MISS_BIT) != 0 
    }
    pub fn dc_l2tlb_hit_2m(&self) -> bool { 
        (self.0 & Self::DC_L2TLB_HIT_2M_BIT) != 0 
    }
    pub fn dc_l1tlb_hit_1g(&self) -> bool { 
        (self.0 & Self::DC_L1TLB_HIT_1G_BIT) != 0 
    }
    pub fn dc_l1tlb_hit_2m(&self) -> bool { 
        (self.0 & Self::DC_L1TLB_HIT_2M_BIT) != 0 
    }
    pub fn dc_l2tlb_miss(&self) -> bool { 
        (self.0 & Self::DC_L2TLB_MISS_BIT) != 0 
    }
    pub fn dc_l1tlb_miss(&self) -> bool { 
        (self.0 & Self::DC_L1TLB_MISS_BIT) != 0 
    }
    pub fn st_op(&self) -> bool { 
        (self.0 & Self::ST_OP_BIT) != 0 
    }
    pub fn ld_op(&self) -> bool { 
        (self.0 & Self::LD_OP_BIT) != 0 
    }
}

/// Representing load/store width values captured by IBS.
///
/// See the IBS_OP_DATA3 entry in the PPR for Family 17h Model 71h.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum IbsMemWidth {
    None    = 0,
    Byte    = 8,
    Word    = 16,
    Dword   = 32,
    Qword   = 64,
    Oword   = 128,
    Yword   = 256,
}
impl From<usize> for IbsMemWidth {
    fn from(x: usize) -> Self {
        match x {
            0x0 => IbsMemWidth::None,
            0x1 => IbsMemWidth::Byte,
            0x2 => IbsMemWidth::Word,
            0x3 => IbsMemWidth::Dword,
            0x4 => IbsMemWidth::Qword,
            0x5 => IbsMemWidth::Oword,
            0x6 => IbsMemWidth::Yword,
            _ => panic!("Invalid/unsupported access width value {}", x),
        }
    }
}


