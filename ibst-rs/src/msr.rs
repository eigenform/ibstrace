use num_enum::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum Msr {

    // ================================================
    // MSRs - MSR 0000_xxxx

    // MCAX0 aliases
    MC0_ADDR                = 0x0000_0000,
    MC0_STATUS              = 0x0000_0001,

    TSC                     = 0x0000_0010,

    APIC_BAR                = 0x0000_001b,

    EBL_CR_POWERON          = 0x0000_002a,

    SPEC_CTL                = 0x0000_0048,

    PATCH_LEVEL             = 0x0000_008b,

    MPERF                   = 0x0000_00e7,
    APERF                   = 0x0000_00e8,

    MTRR_CAP                = 0x0000_00fe,

    SYSENTER_CS             = 0x0000_0174,
    SYSENTER_ESP            = 0x0000_0175,
    SYSENTER_EIP            = 0x0000_0176,

    MCG_CAP                 = 0x0000_0179,
    MCG_STAT                = 0x0000_017a,
    MCG_CTL                 = 0x0000_017b,

    DBG_CTL                 = 0x0000_01d9,

    BR_FROM                 = 0x0000_01db,
    BR_TO                   = 0x0000_01dc,
    LAST_EXCP_FROM_IP       = 0x0000_01dd,
    LAST_EXCP_TO_IP         = 0x0000_01de,

    MTRR_PHYS_BASE_0        = 0x0000_0200,
    MTRR_PHYS_MASK_0        = 0x0000_0201,
    MTRR_PHYS_BASE_1        = 0x0000_0202,
    MTRR_PHYS_MASK_1        = 0x0000_0203,
    MTRR_PHYS_BASE_2        = 0x0000_0204,
    MTRR_PHYS_MASK_2        = 0x0000_0205,
    MTRR_PHYS_BASE_3        = 0x0000_0206,
    MTRR_PHYS_MASK_3        = 0x0000_0207,
    MTRR_PHYS_BASE_4        = 0x0000_0208,
    MTRR_PHYS_MASK_4        = 0x0000_0209,
    MTRR_PHYS_BASE_5        = 0x0000_020a,
    MTRR_PHYS_MASK_5        = 0x0000_020b,
    MTRR_PHYS_BASE_6        = 0x0000_020c,
    MTRR_PHYS_MASK_6        = 0x0000_020d,
    MTRR_PHYS_BASE_7        = 0x0000_020e,
    MTRR_PHYS_MASK_7        = 0x0000_020f,

    MTRR_FIX_64K            = 0x0000_0250,
    MTRR_FIX_16K_0          = 0x0000_0258,
    MTRR_FIX_16K_1          = 0x0000_0259,
    MTRR_FIX_4K_0           = 0x0000_0268,
    MTRR_FIX_4K_1           = 0x0000_0269,
    MTRR_FIX_4K_2           = 0x0000_026a,
    MTRR_FIX_4K_3           = 0x0000_026b,
    MTRR_FIX_4K_4           = 0x0000_026c,
    MTRR_FIX_4K_5           = 0x0000_026d,
    MTRR_FIX_4K_6           = 0x0000_026e,
    MTRR_FIX_4K_7           = 0x0000_026f,

    PAT                     = 0x0000_0277,
    MTRR_DEF_TYPE           = 0x0000_02ff,

    // ================================================
    // MSR 0x0000_0400 - 0x0000_047f 
    // Legacy MCA aliases (32 banks of 4 registers)
    //
    // ...
    //
    //


    // Shadow-stack control
    U_CET                   = 0x0000_06a0,
    S_CET                   = 0x0000_06a2,
    PL0_SSP                 = 0x0000_06a4,
    PL1_SSP                 = 0x0000_06a5,
    PL2_SSP                 = 0x0000_06a6,
    PL3_SSP                 = 0x0000_06a7,
    IST_SSP_ADDR            = 0x0000_06a8,

    // APIC registers
    APIC_ID                 = 0x0000_0802,
    APIC_VERSION            = 0x0000_0803,

    TPR                     = 0x0000_0808,
    ARBITRATION_PRIORITY    = 0x0000_0809,
    PROCESSOR_PRIORITY      = 0x0000_080a,
    EOI                     = 0x0000_080b,
    LDR                     = 0x0000_080d,
    SVR                     = 0x0000_080f,

    ISR0                    = 0x0000_0810,
    ISR1                    = 0x0000_0811,
    ISR2                    = 0x0000_0812,
    ISR3                    = 0x0000_0813,
    ISR4                    = 0x0000_0814,
    ISR5                    = 0x0000_0815,
    ISR6                    = 0x0000_0816,
    ISR7                    = 0x0000_0817,

    TMR0                    = 0x0000_0818,
    TMR1                    = 0x0000_0819,
    TMR2                    = 0x0000_081a,
    TMR3                    = 0x0000_081b,
    TMR4                    = 0x0000_081c,
    TMR5                    = 0x0000_081d,
    TMR6                    = 0x0000_081e,
    TMR7                    = 0x0000_081f,

    IRR0                    = 0x0000_0820,
    IRR1                    = 0x0000_0821,
    IRR2                    = 0x0000_0822,
    IRR3                    = 0x0000_0823,
    IRR4                    = 0x0000_0824,
    IRR5                    = 0x0000_0825,
    IRR6                    = 0x0000_0826,
    IRR7                    = 0x0000_0827,

    ESR                     = 0x0000_0828,

    INTERRUPT_CMD           = 0x0000_0830,

    TIMER_LVT_ENTRY         = 0x0000_0832,
    THERMAL_LVT_ENTRY       = 0x0000_0833,
    PERF_CTR_LVT_ENTRY      = 0x0000_0834,
    LVTLINT0                = 0x0000_0835,
    LVTLINT1                = 0x0000_0836,
    ERR_LVT_ENTRY           = 0x0000_0837,
    TIMER_INITIAL_CNT       = 0x0000_0838,
    TIMER_CURRENT_CNT       = 0x0000_0839,
    TIMER_DIVIDE_CFG        = 0x0000_083e,
    SELF_IPI                = 0x0000_083f,

    EXT_APIC_FEATURE        = 0x0000_0840,
    EXT_APIC_CTL            = 0x0000_0841,
    SPECIFIC_EOI            = 0x0000_0842,

    INTERRUPT_EN0           = 0x0000_0848,
    INTERRUPT_EN1           = 0x0000_0849,
    INTERRUPT_EN2           = 0x0000_084a,
    INTERRUPT_EN3           = 0x0000_084b,
    INTERRUPT_EN4           = 0x0000_084c,
    INTERRUPT_EN5           = 0x0000_084d,
    INTERRUPT_EN6           = 0x0000_084e,
    INTERRUPT_EN7           = 0x0000_084f,

    EXT_INTR_LVT_ENTRY0     = 0x0000_0850,
    EXT_INTR_LVT_ENTRY1     = 0x0000_0851,
    EXT_INTR_LVT_ENTRY2     = 0x0000_0852,
    EXT_INTR_LVT_ENTRY3     = 0x0000_0853,


    // L3 Cache QOS
    L3_QOS_CFG1             = 0x0000_0c81,

    QM_EVTSEL               = 0x0000_0c8d,
    QM_CTR                  = 0x0000_0c8e,
    PQR_ASSOC               = 0x0000_0c8f,
    L3_QOS_ALLOC_MASK_00    = 0x0000_0c90,
    L3_QOS_ALLOC_MASK_01    = 0x0000_0c91,
    L3_QOS_ALLOC_MASK_02    = 0x0000_0c92,
    L3_QOS_ALLOC_MASK_03    = 0x0000_0c93,
    L3_QOS_ALLOC_MASK_04    = 0x0000_0c94,
    L3_QOS_ALLOC_MASK_05    = 0x0000_0c95,
    L3_QOS_ALLOC_MASK_06    = 0x0000_0c96,
    L3_QOS_ALLOC_MASK_07    = 0x0000_0c97,
    L3_QOS_ALLOC_MASK_08    = 0x0000_0c98,
    L3_QOS_ALLOC_MASK_09    = 0x0000_0c99,
    L3_QOS_ALLOC_MASK_10    = 0x0000_0c9a,
    L3_QOS_ALLOC_MASK_11    = 0x0000_0c9b,
    L3_QOS_ALLOC_MASK_12    = 0x0000_0c9c,
    L3_QOS_ALLOC_MASK_13    = 0x0000_0c9d,
    L3_QOS_ALLOC_MASK_14    = 0x0000_0c9e,
    L3_QOS_ALLOC_MASK_15    = 0x0000_0c9f,

    XSS                     = 0x0000_0da0,

    // ================================================
    // MSRs - MSR c000_xxxx

    EFER                    = 0xc000_0080,
    STAR                    = 0xc000_0081,
    LSTAR                   = 0xc000_0082,
    CSTAR                   = 0xc000_0083,
    SF_MASK                 = 0xc000_0084,

    MPERF_RO                = 0xc000_00e7,
    APERF_RO                = 0xc000_00e8,
    IR_PERF_COUNT           = 0xc000_00e9,

    FS_BASE                 = 0xc000_0100,
    GS_BASE                 = 0xc000_0101,
    KERNEL_GS_BASE          = 0xc000_0102,
    TSC_AUX                 = 0xc000_0103,
    TSC_RATIO               = 0xc000_0104,

    PREFETCH_CONTROL        = 0xc000_0108,

    DBG_EXTN_CFG            = 0xc000_010f,

    L3_QOS_BW_CTL_00        = 0xc000_0200,
    L3_QOS_BW_CTL_01        = 0xc000_0201,
    L3_QOS_BW_CTL_02        = 0xc000_0202,
    L3_QOS_BW_CTL_03        = 0xc000_0203,
    L3_QOS_BW_CTL_04        = 0xc000_0204,
    L3_QOS_BW_CTL_05        = 0xc000_0205,
    L3_QOS_BW_CTL_06        = 0xc000_0206,
    L3_QOS_BW_CTL_07        = 0xc000_0207,
    L3_QOS_BW_CTL_08        = 0xc000_0208,
    L3_QOS_BW_CTL_09        = 0xc000_0209,
    L3_QOS_BW_CTL_10        = 0xc000_020a,
    L3_QOS_BW_CTL_11        = 0xc000_020b,
    L3_QOS_BW_CTL_12        = 0xc000_020c,
    L3_QOS_BW_CTL_13        = 0xc000_020d,
    L3_QOS_BW_CTL_14        = 0xc000_020e,
    L3_QOS_BW_CTL_15        = 0xc000_020f,

    MCA_INTR_CFG            = 0xc000_0410,

    // ================================================
    // MSR 0xc000_2000 - 0xc000_23ff
    // MCAX registers (64 banks of 16 registers)

    // MCA Bank - Load/Store Unit
    MCA_CTL_LS              = 0xc000_2000,
    MCA_STATUS_LS           = 0xc000_2001,
    MCA_ADDR_LS             = 0xc000_2002,
    MCA_MISC0_LS            = 0xc000_2003,
    MCA_CONFIG_LS           = 0xc000_2004,
    MCA_IPID_LS             = 0xc000_2005,
    MCA_SYND_LS             = 0xc000_2006,
    MCA_DESTAT_LS           = 0xc000_2008,
    MCA_DEADDR_LS           = 0xc000_2009,

    // MCA Bank - Instruction Fetch Unit
    MCA_CTL_IF              = 0xc000_2010,
    MCA_STATUS_IF           = 0xc000_2011,
    MCA_ADDR_IF             = 0xc000_2012,
    MCA_MISC0_IF            = 0xc000_2013,
    MCA_CONFIG_IF           = 0xc000_2014,
    MCA_IPID_IF             = 0xc000_2015,
    MCA_SYND_IF             = 0xc000_2016,

    // MCA Bank - L2 Cache Unit
    MCA_CTL_L2              = 0xc000_2020,
    MCA_STATUS_L2           = 0xc000_2021,
    MCA_ADDR_L2             = 0xc000_2022,
    MCA_MISC0_L2            = 0xc000_2023,
    MCA_CONFIG_L2           = 0xc000_2024,
    MCA_IPID_L2             = 0xc000_2025,
    MCA_SYND_L2             = 0xc000_2026,
    MCA_DESTAT_L2           = 0xc000_2028,
    MCA_DEADDR_L2           = 0xc000_2029,

    // MCA Bank - Decode Unit
    MCA_CTL_DE              = 0xc000_2030,
    MCA_STATUS_DE           = 0xc000_2031,
    MCA_ADDR_DE             = 0xc000_2032,
    MCA_MISC0_DE            = 0xc000_2033,
    MCA_CONFIG_DE           = 0xc000_2034,
    MCA_IPID_DE             = 0xc000_2035,
    MCA_SYND_DE             = 0xc000_2036,

    // MCA Bank - Execution Unit
    MCA_CTL_EX              = 0xc000_2050,
    MCA_STATUS_EX           = 0xc000_2051,
    MCA_ADDR_EX             = 0xc000_2052,
    MCA_MISC0_EX            = 0xc000_2053,
    MCA_CONFIG_EX           = 0xc000_2054,
    MCA_IPID_EX             = 0xc000_2055,
    MCA_SYND_EX             = 0xc000_2056,

    // MCA Bank - Floating-Point Unit
    MCA_CTL_FP              = 0xc000_2060,
    MCA_STATUS_FP           = 0xc000_2061,
    MCA_ADDR_FP             = 0xc000_2062,
    MCA_MISC0_FP            = 0xc000_2063,
    MCA_CONFIG_FP           = 0xc000_2064,
    MCA_IPID_FP             = 0xc000_2065,
    MCA_SYND_FP             = 0xc000_2066,

    // MCA Bank - L3 Cache Unit
    // TODO: aliases?
    MCA_CTL_L3              = 0xc000_20e0,
    MCA_STATUS_L3           = 0xc000_20e1,
    MCA_ADDR_L3             = 0xc000_20e2,
    MCA_MISC0_L3            = 0xc000_20e3,
    MCA_CONFIG_L3           = 0xc000_20e4,
    MCA_IPID_L3             = 0xc000_20e5,
    MCA_SYND_L3             = 0xc000_20e6,
    MCA_DESTAT_L3           = 0xc000_20e8,
    MCA_DEADDR_L3           = 0xc000_20e9,

    // MCA Bank - Coherent Slave
    // TODO: aliases?
    MCA_CTL_CS              = 0xc000_2150,
    MCA_STATUS_CS           = 0xc000_2151,
    MCA_ADDR_CS             = 0xc000_2152,
    MCA_MISC0_CS            = 0xc000_2153,
    MCA_CONFIG_CS           = 0xc000_2154,
    MCA_IPID_CS             = 0xc000_2155,
    MCA_SYND_CS             = 0xc000_2156,
    MCA_DESTAT_CS           = 0xc000_2158,
    MCA_DEADDR_CS           = 0xc000_2159,

    // MCA Bank - Power Management/Interrupts/etc
    MCA_CTL_PIE             = 0xc000_21b0,
    MCA_STATUS_PIE          = 0xc000_21b1,
    MCA_ADDR_PIE            = 0xc000_21b2,
    MCA_MISC0_PIE           = 0xc000_21b3,
    MCA_CONFIG_PIE          = 0xc000_21b4,
    MCA_IPID_PIE            = 0xc000_21b5,
    MCA_SYND_PIE            = 0xc000_21b6,
    MCA_DESTAT_PIE          = 0xc000_21b8,
    MCA_DEADDR_PIE          = 0xc000_21b9,

    // MCA Bank - Unified Memory Controller
    MCA_CTL_UMC             = 0xc000_2120,
    MCA_STATUS_UMC          = 0xc000_2121,
    MCA_ADDR_UMC            = 0xc000_2122,
    MCA_MISC0_UMC           = 0xc000_2123,
    MCA_CONFIG_UMC          = 0xc000_2124,
    MCA_IPID_UMC            = 0xc000_2125,
    MCA_SYND_UMC            = 0xc000_2126,
    MCA_DESTAT_UMC          = 0xc000_2128,
    MCA_DEADDR_UMC          = 0xc000_2129,
    MCA_MISC1_UMC           = 0xc000_212a,

    // MCA Bank - Parameter Block
    MCA_CTL_PB              = 0xc000_21a0,
    MCA_STATUS_PB           = 0xc000_21a1,
    MCA_ADDR_PB             = 0xc000_21a2,
    MCA_MISC0_PB            = 0xc000_21a3,
    MCA_CONFIG_PB           = 0xc000_21a4,
    MCA_IPID_PB             = 0xc000_21a5,
    MCA_SYND_PB             = 0xc000_21a6,

    // MCA Bank - Platform Security Processor
    MCA_CTL_PSP             = 0xc000_2190,
    MCA_STATUS_PSP          = 0xc000_2191,
    MCA_ADDR_PSP            = 0xc000_2192,
    MCA_MISC0_PSP           = 0xc000_2193,
    MCA_CONFIG_PSP          = 0xc000_2194,
    MCA_IPID_PSP            = 0xc000_2195,
    MCA_SYND_PSP            = 0xc000_2196,

    // MCA Bank - System Management Controller Unit
    MCA_CTL_SMU             = 0xc000_2180,
    MCA_STATUS_SMU          = 0xc000_2181,
    MCA_ADDR_SMU            = 0xc000_2182,
    MCA_MISC0_SMU           = 0xc000_2183,
    MCA_CONFIG_SMU          = 0xc000_2184,
    MCA_IPID_SMU            = 0xc000_2185,
    MCA_SYND_SMU            = 0xc000_2186,

    // MCA Bank - Microprocessor5 Management Controller
    MCA_CTL_MP5             = 0xc000_20f0,
    MCA_STATUS_MP5          = 0xc000_20f1,
    MCA_ADDR_MP5            = 0xc000_20f2,
    MCA_MISC0_MP5           = 0xc000_20f3,
    MCA_CONFIG_MP5          = 0xc000_20f4,
    MCA_IPID_MP5            = 0xc000_20f5,
    MCA_SYND_MP5            = 0xc000_20f6,

    // MCA Bank - Northbridge I/O Unit
    MCA_CTL_NBIO            = 0xc000_2160,
    MCA_STATUS_NBIO         = 0xc000_2161,
    MCA_ADDR_NBIO           = 0xc000_2162,
    MCA_MISC0_NBIO          = 0xc000_2163,
    MCA_CONFIG_NBIO         = 0xc000_2164,
    MCA_IPID_NBIO           = 0xc000_2165,
    MCA_SYND_NBIO           = 0xc000_2166,
    MCA_DESTAT_NBIO         = 0xc000_2168,
    MCA_DEADDR_NBIO         = 0xc000_2169,

    // MCA Bank - PCIe Root Port
    MCA_CTL_PCIE            = 0xc000_2170,
    MCA_STATUS_PCIE         = 0xc000_2171,
    MCA_ADDR_PCIE           = 0xc000_2172,
    MCA_MISC0_PCIE          = 0xc000_2173,
    MCA_CONFIG_PCIE         = 0xc000_2174,
    MCA_IPID_PCIE           = 0xc000_2175,
    MCA_SYND_PCIE           = 0xc000_2176,
    MCA_DESTAT_PCIE         = 0xc000_2178,
    MCA_DEADDR_PCIE         = 0xc000_2179,



    // ================================================
    // MSRs - MSR c001_xxxx

    PERF_LEGACY_CTL0        = 0xc000_0000,
    PERF_LEGACY_CTL1        = 0xc000_0001,
    PERF_LEGACY_CTL2        = 0xc000_0002,
    PERF_LEGACY_CTL3        = 0xc000_0003,
    PERF_LEGACY_CTR0        = 0xc000_0004,
    PERF_LEGACY_CTR1        = 0xc000_0005,
    PERF_LEGACY_CTR2        = 0xc000_0006,
    PERF_LEGACY_CTR3        = 0xc000_0007,

    SYS_CFG                 = 0xc001_0010,

    HWCR                    = 0xc001_0015,
    IORR_BASE_0             = 0xc001_0016,
    IORR_MASK_0             = 0xc001_0017,
    IORR_BASE_1             = 0xc001_0018,
    IORR_MASK_1             = 0xc001_0019,
    TOP_MEM                 = 0xc001_001a,

    TOM2                    = 0xc001_001d,

    PROC_NAME_STR_0         = 0xc001_0030,
    PROC_NAME_STR_1         = 0xc001_0031,
    PROC_NAME_STR_2         = 0xc001_0032,
    PROC_NAME_STR_3         = 0xc001_0033,
    PROC_NAME_STR_4         = 0xc001_0034,
    PROC_NAME_STR_5         = 0xc001_0035,

    SMI_ON_IO_TRAP_CTL_STS  = 0xc001_0054,
    INT_PEND                = 0xc001_0055,
    SMI_TRIG_IO_CYCLE       = 0xc001_0056,

    MMIO_CFG_BASE_ADDR      = 0xc001_0058,

    PSTATE_CUR_LIM          = 0xc001_0061,
    PSTATE_CTRL             = 0xc001_0062,
    PSTATE_STATUS           = 0xc001_0063,
    PSTATE_DEF_0            = 0xc001_0064,
    PSTATE_DEF_1            = 0xc001_0065,
    PSTATE_DEF_2            = 0xc001_0066,
    PSTATE_DEF_3            = 0xc001_0067,
    PSTATE_DEF_4            = 0xc001_0068,
    PSTATE_DEF_5            = 0xc001_0069,
    PSTATE_DEF_6            = 0xc001_006a,
    PSTATE_DEF_7            = 0xc001_006b,

    CSTATE_BASE_ADDR        = 0xc001_0073,
    CPU_WDT_CFG             = 0xc001_0074,

    SMM_BASE                = 0xc001_0111,
    SMM_ADDR                = 0xc001_0112,
    SMM_MASK                = 0xc001_0113,
    VM_CR                   = 0xc001_0114,
    IGNNE                   = 0xc001_0115,
    SMM_CTL                 = 0xc001_0116,
    VM_HSAVE_PA             = 0xc001_0117,
    SVM_LOCK_KEY            = 0xc001_0118,
    LOCAL_SMI_STATUS        = 0xc001_011a,
    AVIC_DOORBELL           = 0xc001_011b,

    VMPAGE_FLUSH            = 0xc001_011e,

    GHCB                    = 0xc001_0130,
    SEV_STATUS              = 0xc001_0131,
    LS_RMP_BASE             = 0xc001_0132,
    LS_RMP_END              = 0xc001_0133,

    OSVW_ID_LENGTH          = 0xc001_0140,
    OSWV_STATUS             = 0xc001_0141,

    PERF_CTL_0              = 0xc001_0200,
    PERF_CTR_0              = 0xc001_0201,
    PERF_CTL_1              = 0xc001_0202,
    PERF_CTR_1              = 0xc001_0203,
    PERF_CTL_2              = 0xc001_0204,
    PERF_CTR_2              = 0xc001_0205,
    PERF_CTL_3              = 0xc001_0206,
    PERF_CTR_3              = 0xc001_0207,
    PERF_CTL_4              = 0xc001_0208,
    PERF_CTR_4              = 0xc001_0209,
    PERF_CTL_5              = 0xc001_020a,
    PERF_CTR_5              = 0xc001_020b,

    CH_L3_PMC_CFG0          = 0xc001_0230,
    CH_L3_PMC0              = 0xc001_0231,
    CH_L3_PMC_CFG1          = 0xc001_0232,
    CH_L3_PMC1              = 0xc001_0233,
    CH_L3_PMC_CFG2          = 0xc001_0234,
    CH_L3_PMC2              = 0xc001_0235,
    CH_L3_PMC_CFG3          = 0xc001_0236,
    CH_L3_PMC3              = 0xc001_0237,
    CH_L3_PMC_CFG4          = 0xc001_0238,
    CH_L3_PMC4              = 0xc001_0239,
    CH_L3_PMC_CFG5          = 0xc001_023a,
    CH_L3_PMC5              = 0xc001_023b,

    DF_PERF_CTL0            = 0xc001_0240,
    DF_PERF_CTR0            = 0xc001_0241,
    DF_PERF_CTL1            = 0xc001_0242,
    DF_PERF_CTR1            = 0xc001_0243,
    DF_PERF_CTL2            = 0xc001_0244,
    DF_PERF_CTR2            = 0xc001_0245,
    DF_PERF_CTL3            = 0xc001_0246,
    DF_PERF_CTR3            = 0xc001_0247,

    CSTATE_POLICY           = 0xc001_0294,

    CSTATE_CONFIG           = 0xc001_0296,

    RAPL_PWR_UNIT           = 0xc001_0299,
    CORE_ENERGY_STAT        = 0xc001_029a,
    PKG_ENERGY_STAT         = 0xc001_029b,

    CPPC_CAPABILITY1        = 0xc001_02b0,
    CPPC_ENABLE             = 0xc001_02b1,
    CPPC_CAPABILITY2        = 0xc001_02b2,
    CPPC_REQUEST            = 0xc001_02b3,
    CPPC_STATUS             = 0xc001_02b4,

    PPIN_CTL                = 0xc001_02f0,
    PPIN                    = 0xc001_02f1,

    SAMP_BR_FROM_00         = 0xc001_0300,
    SAMP_BR_TO_00           = 0xc001_0301,
    SAMP_BR_FROM_01         = 0xc001_0302,
    SAMP_BR_TO_01           = 0xc001_0303,
    SAMP_BR_FROM_02         = 0xc001_0304,
    SAMP_BR_TO_02           = 0xc001_0305,
    SAMP_BR_FROM_03         = 0xc001_0306,
    SAMP_BR_TO_03           = 0xc001_0307,
    SAMP_BR_FROM_04         = 0xc001_0308,
    SAMP_BR_TO_04           = 0xc001_0309,
    SAMP_BR_FROM_05         = 0xc001_030a,
    SAMP_BR_TO_05           = 0xc001_030b,
    SAMP_BR_FROM_06         = 0xc001_030c,
    SAMP_BR_TO_06           = 0xc001_030d,
    SAMP_BR_FROM_07         = 0xc001_030e,
    SAMP_BR_TO_07           = 0xc001_030f,
    SAMP_BR_FROM_08         = 0xc001_0310,
    SAMP_BR_TO_08           = 0xc001_0311,
    SAMP_BR_FROM_09         = 0xc001_0312,
    SAMP_BR_TO_09           = 0xc001_0313,
    SAMP_BR_FROM_10         = 0xc001_0314,
    SAMP_BR_TO_10           = 0xc001_0315,
    SAMP_BR_FROM_11         = 0xc001_0316,
    SAMP_BR_TO_11           = 0xc001_0317,
    SAMP_BR_FROM_12         = 0xc001_0318,
    SAMP_BR_TO_12           = 0xc001_0319,
    SAMP_BR_FROM_13         = 0xc001_031a,
    SAMP_BR_TO_13           = 0xc001_031b,
    SAMP_BR_FROM_14         = 0xc001_031c,
    SAMP_BR_TO_14           = 0xc001_031d,
    SAMP_BR_FROM_15         = 0xc001_031e,
    SAMP_BR_TO_15           = 0xc001_031f,

    // ================================================
    // MSR 0xc000_0400 - 0xc000_04ff
    // MCA_CTL_MASK registers

    MCA_CTL_MASK_LS         = 0xc001_0400,
    MCA_CTL_MASK_IF         = 0xc001_0401,
    MCA_CTL_MASK_L2         = 0xc001_0402,
    MCA_CTL_MASK_DE         = 0xc001_0403,
    MCA_CTL_MASK_EX         = 0xc001_0405,
    MCA_CTL_MASK_FP         = 0xc001_0406,
    MCA_CTL_MASK_L3         = 0xc001_040e,
    MCA_CTL_MASK_MP5        = 0xc001_040f,
    MCA_CTL_MASK_UMC        = 0xc001_0412,
    MCA_CTL_MASK_CS         = 0xc001_0415,
    MCA_CTL_MASK_NBIO       = 0xc001_0416,
    MCA_CTL_MASK_PCIE       = 0xc001_0417,
    MCA_CTL_MASK_SMU        = 0xc001_0418,
    MCA_CTL_MASK_PSP        = 0xc001_0419,
    MCA_CTL_MASK_PB         = 0xc001_041a,
    MCA_CTL_MASK_PIE        = 0xc001_041b,

    CPUID_7_FEATURES        = 0xc001_1002,
    CPUID_PWR_THERM         = 0xc001_1003,
    CPUID_FEATURES          = 0xc001_1004,
    CPUID_EXT_FEATURES      = 0xc001_1005,

    DR1_ADDR_MASK           = 0xc001_1019,
    DR2_ADDR_MASK           = 0xc001_101a,
    DR3_ADDR_MASK           = 0xc001_101b,

    // ================================================
    // Defeature/special configuration registers
    // NOTE: Many of these are partially documented in illumos

    MCODE_CTL               = 0xc001_1000,
    LS_CFG                  = 0xc001_1020,
    IC_CFG                  = 0xc001_1021,
    DC_CFG                  = 0xc001_1022,
    TW_CFG                  = 0xc001_1023,

    DR0_ADDR_MASK           = 0xc001_1027,
    FP_CFG                  = 0xc001_1028,
    DE_CFG                  = 0xc001_1029,
    L2_CFG                  = 0xc001_102a,
    CH_L2_PF_CFG            = 0xc001_102b,
    LS_CFG2                 = 0xc001_102d,
    BP_CFG                  = 0xc001_102e,
    CH_L2_RANGE_LOCK0       = 0xc001_102f,

    // IBS

    IBS_FETCH_CTL           = 0xc001_1030,
    IBS_FETCH_LINADDR       = 0xc001_1031,
    IBS_FETCH_PHYADDR       = 0xc001_1032,
    IBS_OP_CTL              = 0xc001_1033,
    IBS_OP_RIP              = 0xc001_1034,
    IBS_OP_DATA             = 0xc001_1035,
    IBS_OP_DATA2            = 0xc001_1036,
    IBS_OP_DATA3            = 0xc001_1037,
    IBS_DC_LINADDR          = 0xc001_1038,
    IBS_DC_PHYADDR          = 0xc001_1039,
    IBS_CTL                 = 0xc001_103a,
    BP_IBSTGT_RIP           = 0xc001_103b,
    IC_IBS_EXTD_CTL         = 0xc001_103c,

    // Dynamic power management?

    DPM_CFG                 = 0xc001_1074,
    DPM_WAC_ACC_INDEX       = 0xc001_1076,
    DPM_WAC_DATA            = 0xc001_1077,
    DPM_ACC_DATA            = 0xc001_1078,

    CH_L3_CFG0              = 0xc001_1092,
    CH_L3_CFG1              = 0xc001_1093,
    CH_L3_RANGE_LOCK_BASE   = 0xc001_1095,
    CH_L3_RANGE_LOCK_MAX    = 0xc001_1096,
    CH_L3_XI_CFG0           = 0xc001_1097,
    CH_L3_QOS_CFG2          = 0xc001_1098,
    CH_L3_RANGE_LOCK_WAY    = 0xc001_109a,

    PSP_ADDR                = 0xc001_10a2,

    FEATURE_EXT_ID          = 0xc001_10dc,
    SVM_REV_FEAT_ID         = 0xc001_10dd,
    FEATURE_EXT2_EAX        = 0xc001_10de,
    //STRUCT_EX_FEAT_ID     = 0xc001_10df,

    CH_L2_CFG1              = 0xc001_10e2,
    DE_CFG2                 = 0xc001_10e3,

}


