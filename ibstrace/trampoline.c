// SPDX-License-Identifier: GPL-2.0

#include <linux/smp.h>
#include <linux/delay.h>
#include <asm/msr.h>
#include <linux/mutex.h>

#include <ibstrace.h>
#include "state.h"
#include "msr.h"

// FIXME: Consider saving the return value from measured code so we can send 
// it back to userspace

extern struct ibstrace_state state;

int __trampoline_start(void *code_ptr, void *scratch_page_vaddr);
int __precise_trampoline_start(
	void *code_ptr, 
	void *scratch_page_vaddr,
	__u64 offset
);

// Cursed hack #3: 
// Trampoline into user-submitted code. This function is guaranteed to execute 
// on the target core when we call it using smp_call_function_single_async().
// See kernel/trampoline_asm.S for more notes on why we do it like this.
void trampoline(void *info)
{
	int res = -1;

	res = __trampoline_start(state.code_buf, state.scratch_page);

	// This lock is aquired in ibstrace_ioctl() just before we use
	// smp_call_function_single_async() to call this function.

	mutex_unlock(&state.in_use);
}

void precise_trampoline(void *info)
{
	int res = -1;

	struct ibstrace_precise_msg *arg = (struct ibstrace_precise_msg*)info;

	res = __precise_trampoline_start(
			state.code_buf, 
			arg->ptr,
			arg->offset
	);

	// This lock is aquired in ibstrace_ioctl() just before we use
	// smp_call_function_single_async() to call this function.

	mutex_unlock(&state.in_use);
}


