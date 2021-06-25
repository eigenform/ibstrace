// SPDX-License-Identifier: GPL-2.0

#include <asm/apic.h>
#include <asm/apicdef.h>
#include <asm/perf_event.h>

#include "apic.h"

// Read the LVT offset from IBSCTL (otherwise, return EINVAL).
static inline int get_ibs_lvt_offset(void)
{
	u64 val;

	rdmsrl(MSR_AMD64_IBSCTL, val);
	if (!(val & IBSCTL_LVT_OFFSET_VALID))
		return -EINVAL;

	return val & IBSCTL_LVT_OFFSET_MASK;
}


// Enable IBS NMIs on the local APIC.
void ibs_apic_init(void *info)
{
	int offset;

	preempt_disable();
	offset = get_ibs_lvt_offset();
	if (offset < 0)
		goto failed;

	if (!setup_APIC_eilvt(offset, 0, APIC_EILVT_MSG_NMI, 0)) {
		pr_info("ibstrace: APIC initialized for cpu #%d\n",
			smp_processor_id());
		goto out;
	}

failed:
	pr_warn("ibstrace: APIC setup failed for cpu #%d\n",
		smp_processor_id());
out:
	preempt_enable();
}

// Disable IBS NMIs on the local APIC.
void ibs_apic_exit(void *info)
{
	int offset;

	preempt_disable();
	offset = get_ibs_lvt_offset();
	if (offset >= 0)
		setup_APIC_eilvt(offset, 0, APIC_EILVT_MSG_FIX, 1);

	pr_info("ibstrace: IBS APIC teardown for cpu #%d\n",
		smp_processor_id());

	preempt_enable();
}


