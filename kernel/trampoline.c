// SPDX-License-Identifier: GPL-2.0

#include <linux/smp.h>
#include <linux/delay.h>
#include <asm/msr.h>

#include <ibstrace.h>
#include "state.h"
#include "msr.h"

extern struct ibstrace_state state;
int __trampoline_start(void *code_ptr, unsigned long paddr);


// Cursed hack #3: 
// Trampoline into user-submitted code. This function is guaranteed to execute 
// on the target core when we call it using smp_call_function_single_async().
// See kernel/trampoline_asm.S for more notes on why we do it like this.
void trampoline(void *info)
{
	u64 efer;
	int res = -1;

	// Enable SVM by setting the bit in the EFER
	rdmsrl(MSR_EFER, efer);
	wrmsrl(MSR_EFER, efer | EFER_SVME);
	rdmsrl(MSR_EFER, efer);
	if ((efer & EFER_SVME) == 0) {
		pr_err("ibstrace: couldn't enable SVM\n");
		goto end;
	}

	res = __trampoline_start(state.code_buf, state.scratch_page_paddr);

	// Disable SVM
	rdmsrl(MSR_EFER, efer);
	wrmsrl(MSR_EFER, efer & ~EFER_SVME);

	// This lock is aquired in ibstrace_ioctl() just before we use
	// smp_call_function_single_async() to call this function.
end:
	mutex_unlock(&state.in_use);
}


