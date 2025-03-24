// SPDX-License-Identifier: GPL-2.0

#define IBS_OP_CTL				0xc0011033
#define IBS_OP_RIP				0xc0011034
#define IBS_OP_DATA				0xc0011035
#define IBS_OP_DATA2			0xc0011036
#define IBS_OP_DATA3			0xc0011037
#define IBS_DC_LIN_AD			0xc0011038
#define IBS_DC_PHYS_AD			0xc0011039
#define IBS_CONTROL				0xc001103a
#define BP_IBSTGT_RIP			0xc001103b

#define IBS_LVT_OFFSET_VALID	(1ULL << 8)
#define IBS_LVT_OFFSET_MASK		(0xf)


// IBS_OP_CTL
//
//   63       55       47       39       
//   v        v        v        v        
//   .....ccc cccccccc cccccccc cccccccc 
//
//   31       23       15       07
//   v        v        v        v
//   .....mmm mmmmtve. mmmmmmmm mmmmmmmm
//
//		. - reserved		c - IbsOpCurCnt		m - IbsOpMaxCnt
//		t - IbsOpCntCtl		v - IbsOpVal		e - IbsOpEn
//

// Bitmask for the current counter value
#define IBS_OP_CUR_CNT			(0x07ffffffULL << 32)
// Bitmask for the current counter value (excluding the bottom 4 bits)
#define IBS_OP_CUR_CNT_23		(0x07fffff0ULL << 32)
// Bitmask for the bottom 4 bits of the current counter value
#define IBS_OP_CUR_CNT_RAND		(0x0000000fULL << 32)

// Bitmask for the max counter value
#define IBS_OP_MAX_CNT			(0x07f0ffffULL)

#define IBS_OP_CNT_CTL			(1ULL << 19)
#define IBS_OP_VAL				(1ULL << 18)
#define IBS_OP_EN				(1ULL << 17)

