// SPDX-License-Identifier: GPL-2.0

#include <linux/smp.h>
#include <linux/delay.h>
#include <asm/msr.h>

#include <ibstrace.h>
#include "state.h"
#include "msr.h"

extern struct ibstrace_state state;

// Enable sampling on the current CPU core.
static inline void enable_ibs(void)
{
	u64 ibs_op_ctl;
	rdmsrl(IBS_OP_CTL, ibs_op_ctl);
	pr_info("ibstrace: initial IBS_OP_CTL %016llx\n", ibs_op_ctl);

	ibs_op_ctl |= IBS_OP_CNT_CTL;
	ibs_op_ctl |= (0x0100ULL & IBS_OP_MAX_CNT);
	ibs_op_ctl |= IBS_OP_EN;
	pr_info("ibstrace: start IBS_OP_CTL %016llx\n", ibs_op_ctl);
	wrmsrl(IBS_OP_CTL, ibs_op_ctl);
}


// Disable sampling on the current CPU core.
static inline void disable_ibs(void)
{
	// jlgreathouse/AMD_IBS_Toolkit uses this gadget to wait for any hanging
	// NMIs before clearing the IBS_OP_VAL register, although I'm not sure
	// how necessary this is in our case

	wrmsrl(IBS_OP_CTL, IBS_OP_VAL);
	udelay(1);
	wrmsrl(IBS_OP_CTL, 0ULL);
}

// Cursed hack #3: 
// Trampoline into user-submitted code. This function is guaranteed to execute 
// on the target core when we call it using smp_call_function_single_async().
void trampoline(void *info)
{
	int res;
	pr_info("ibstrace: trampoline on cpu #%d\n", smp_processor_id());

#ifndef QEMU_BUILD
	enable_ibs();
#endif // QEMU_BUILD

	asm(
		// Push the callee-save registers onto the stack
		"push %%rbx\n"
		"push %%rbp\n"
		"push %%r12\n"
		"push %%r13\n"
		"push %%r14\n"
		"push %%r15\n"
		"pushfq\n"

		// Indirect branch to user code
		"call *%%rax\n"

		// Pop the callee-save registers from the stack
		"popfq\n"
		"pop %%r15\n"
		"pop %%r14\n"
		"pop %%r13\n"
		"pop %%r12\n"
		"pop %%rbp\n"
		"pop %%rbx\n"

		: "=a"(res)				// Input pointer in %rax
		: "a"(state.code_buf)	// Output return value in %rax
	);

#ifndef QEMU_BUILD
	disable_ibs();
#endif // QEMU_BUILD

	pr_info("ibstrace: trampoline returned %d\n", res);

	// This lock is aquired in ibstrace_ioctl() just before we use
	// smp_call_function_single_async() to call this function.

	mutex_unlock(&state.in_use);
}


