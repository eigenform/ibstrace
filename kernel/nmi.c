
#include <linux/nmi.h>
#include <asm/msr-index.h>
#include <asm/perf_event.h>


// The handler for IBS non-maskable interrupts. 
int ibs_nmi_handler(unsigned int cmd, struct pt_regs *regs)
{
	u64 ibs_op_ctl;
	rdmsrl(MSR_AMD64_IBSOPCTL, ibs_op_ctl);

	// If IBS is the source of this NMI, collect the data for this sample
	// and mark the NMI as handled.

	if (ibs_op_ctl & IBS_OP_VAL) {
		pr_info("ibstrace: NMI on CPU #%d\n", smp_processor_id());
		return NMI_HANDLED;
	}

	// If we reach here, we probably caught an NMI from somewhere else,
	// and we want to avoid marking it as handled.

	return NMI_DONE;
}



