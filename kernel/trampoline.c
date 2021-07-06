// SPDX-License-Identifier: GPL-2.0

#include <linux/smp.h>
#include <linux/delay.h>
#include <asm/msr.h>

#include <ibstrace.h>
#include "state.h"
#include "msr.h"

extern struct ibstrace_state state;
int __trampoline_start(void *code_ptr, u64 xloops);


// Cursed hack #3: 
// Trampoline into user-submitted code. This function is guaranteed to execute 
// on the target core when we call it using smp_call_function_single_async().
// See kernel/trampoline_asm.S for more notes on why we do it like this.
void trampoline(void *info)
{
	int res = -1;

	// Plagarized from __const_udelay()
	unsigned long xloops = 0x10c7 * 4;
	unsigned long lpj = this_cpu_read(cpu_info.loops_per_jiffy);
	int d0;
	asm("mull %%edx"
		:"=d" (xloops), "=&a" (d0)
		:"1" (xloops), "0" (lpj * (HZ / 4)));

	res = __trampoline_start(state.code_buf, xloops);
	pr_info("ibstrace: trampoline returned %d\n", res);

	// This lock is aquired in ibstrace_ioctl() just before we use
	// smp_call_function_single_async() to call this function.

	mutex_unlock(&state.in_use);
}


