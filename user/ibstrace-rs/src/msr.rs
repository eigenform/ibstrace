

/// Store/load width values captured by IBS
pub enum IbsMemWidth {
    None = 0x0,
    Byte = 0x1,
    Word = 0x2,
    Dword = 0x3,
    Qword = 0x4,
    Oword = 0x5,
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
            _ => panic!("unknown width value {}", x),
        }
    }
}

/// IBS_OP_CTL
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct IbsOpCtl(pub usize);
impl IbsOpCtl {
    pub fn cur_cnt(self) -> usize { (self.0 & 0x07ff_ffff_0000_0000) >> 32 }
}

/// IBS_OP_DATA
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct IbsOpData(pub usize);
impl IbsOpData {
    pub fn op_microcode(self) -> bool {(self.0 & 0x0000_0100_0000_0000) != 0 }
    pub fn op_brn_fuse(self) -> bool { (self.0 & 0x0000_0080_0000_0000) != 0 }
    pub fn rip_invalid(self) -> bool { (self.0 & 0x0000_0040_0000_0000) != 0 }
    pub fn op_brn_ret(self) -> bool {  (self.0 & 0x0000_0020_0000_0000) != 0 }
    pub fn op_brn_misp(self) -> bool { (self.0 & 0x0000_0010_0000_0000) != 0 }
    pub fn op_brn_taken(self) -> bool {(self.0 & 0x0000_0008_0000_0000) != 0 }
    pub fn op_return(self) -> bool {   (self.0 & 0x0000_0004_0000_0000) != 0 }
    pub fn tag_to_reg_ctr(self) -> usize { 
        (self.0 & 0x0000_0000_ffff_0000) >> 16 
    }
    pub fn comp_to_ret_ctr(self) -> usize { 
        self.0 & 0x0000_0000_0000_ffff 
    }
}

/// IBS_OP_DATA2
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct IbsOpData2(pub usize);

/// IBS_OP_DATA3
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct IbsOpData3(pub usize);
impl IbsOpData3 {
    pub fn tlb_refill_lat(self) -> usize { (self.0 & 0xffff_0000_0000_0000) >> 48 }
    pub fn dc_miss_lat(self) -> usize { (self.0 & 0x0000_ffff_0000_0000) >> 32 }
    pub fn op_dc_miss_open_mem_reqs(self) -> usize { (self.0 & 0x0000_0000_fc00_0000) >> 26 }
    pub fn op_mem_width(self) -> IbsMemWidth {
        IbsMemWidth::from((self.0 & 0x0000_0000_03c0_0000) >> 22)
    }

    pub fn sw_pf(self) -> bool { (self.0 & 0x0000_0000_0020_0000) != 0 }
    pub fn dc_l2_miss(self) -> bool { (self.0 & 0x0000_0000_0010_0000) != 0 }
    pub fn dc_l2tlb_hit_1g(self) -> bool { (self.0 & 0x0000_0000_0008_0000) != 0 }

    pub fn dc_phy_addr_valid(self) -> bool { (self.0 & 0x0000_0000_0004_0000) != 0 }
    pub fn dc_lin_addr_valid(self) -> bool { (self.0 & 0x0000_0000_0002_0000) != 0 }
    pub fn dc_miss_no_mab_alloc(self) -> bool { (self.0 & 0x0000_0000_0001_0000) != 0 }

    pub fn dc_locked_op(self) -> bool { (self.0 & 0x0000_0000_0000_8000) != 0 }
    pub fn dc_uc_mem_acc(self) -> bool { (self.0 & 0x0000_0000_0000_4000) != 0 }
    pub fn dc_wc_mem_acc(self) -> bool { (self.0 & 0x0000_0000_0000_2000) != 0 }

    pub fn dc_mis_acc(self) -> bool { (self.0 & 0x0000_0000_0000_0100) != 0 }
    pub fn dc_miss(self) -> bool { (self.0 & 0x0000_0000_0000_0080) != 0 }
    pub fn dc_l2tlb_hit_2m(self) -> bool { (self.0 & 0x0000_0000_0000_0040) != 0 }
    pub fn dc_l1tlb_hit_1g(self) -> bool { (self.0 & 0x0000_0000_0000_0020) != 0 }
    pub fn dc_l1tlb_hit_2m(self) -> bool { (self.0 & 0x0000_0000_0000_0010) != 0 }
    pub fn dc_l2tlb_miss(self) -> bool { (self.0 & 0x0000_0000_0000_0008) != 0 }
    pub fn dc_l1tlb_miss(self) -> bool { (self.0 & 0x0000_0000_0000_0004) != 0 }
    pub fn st_op(self) -> bool { (self.0 & 0x0000_0000_0000_0002) != 0 }
    pub fn ld_op(self) -> bool { (self.0 & 0x0000_0000_0000_0001) != 0 }
}

/// IBS_OP_DATA4
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct IbsOpData4(pub usize);


