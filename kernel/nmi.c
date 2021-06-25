// SPDX-License-Identifier: GPL-2.0

#include <linux/nmi.h>

#include <ibstrace.h>
#include "state.h"
#include "msr.h"

extern struct ibstrace_state state;

static u64 lsfr4(void)
{
	static u64 tmp = 0xdead;
	u64 bit;
	bit = ((tmp >> 0) ^ (tmp >> 2) ^ (tmp >> 3) ^ (tmp >> 5)) & 1;
	tmp = (tmp >> 1) | (bit << 15);
	return tmp & 0xf;
}

//static void read_sample_data(struct sample *sample, struct pt_regs *regs)
//{
//	rdmsrl(IBS_OP_DATA, sample->op_data);
//	rdmsrl(IBS_OP_DATA2, sample->op_data2);
//	rdmsrl(IBS_OP_DATA3, sample->op_data3);
//	rdmsrl(IBS_DC_LIN_AD, sample->dc_lin_addr);
//	rdmsrl(IBS_DC_PHYS_AD, sample->dc_phys_addr);
//	rdmsrl(IBS_OP_RIP, sample->op_rip);
//	sample->cpu = smp_processor_id();
//	sample->kernel = !user_mode(regs);
//}

// The handler for IBS non-maskable interrupts. 
int ibs_nmi_handler(unsigned int cmd, struct pt_regs *regs)
{
	u64 ibs_op_ctl;
	rdmsrl(IBS_OP_CTL, ibs_op_ctl);

	// If the sample valid bit is set, this is an IBS NMI
	if (ibs_op_ctl & IBS_OP_VAL) {

		// If the IBS_OP_MAX_CNT is zeroed out, skip this sample
		if (!(ibs_op_ctl & IBS_OP_MAX_CNT)) {
			return NMI_HANDLED; 
		}

		// Collect the sample
		atomic_long_inc(&state.samples_collected);

		// Reconfigure IBS_OP_CTL so we can handle another sample
		ibs_op_ctl &= ~IBS_OP_VAL;
		ibs_op_ctl &= ~IBS_OP_CUR_CNT_RAND;
		ibs_op_ctl |= ((lsfr4() << 32) & IBS_OP_CUR_CNT_RAND); 
		wrmsrl(IBS_OP_CTL, ibs_op_ctl);

		return NMI_HANDLED;
	}

	// If we reach here, we caught an NMI that we aren't responsible for,
	// so we need to avoid marking it as handled
	return NMI_DONE;
}


