// SPDX-License-Identifier: GPL-2.0

// Notes:
//	1. User issues ioctl(IBSTRACE_CMD_WRITE, ...)
//	2. ibstrace_ioctl() writes to code_buf and uses smp_call_function_single()
//	   to call trampoline() on the target CPU

#include <linux/module.h>
#include <linux/slab.h>
#include <linux/vmalloc.h>

#include <linux/kallsyms.h>
#include <linux/kprobes.h>

#include <linux/device.h>
#include <linux/miscdevice.h>

#include "ibstrace.h"
#include "apic.h"
#include "fops.h"



struct sample *sample_buf = NULL;

u8 *code_buf = NULL;
u64 code_buf_len = 0;

//DEFINE_MUTEX(global_mutex);

static int (*set_memory_x)(unsigned long, int) = NULL;
static int (*set_memory_nx)(unsigned long, int) = NULL;


static const struct file_operations ibstrace_fops = {
	.owner				= THIS_MODULE,
	.unlocked_ioctl		= ibstrace_ioctl,
};
static struct miscdevice ibstrace_dev = {
	.minor				= MISC_DYNAMIC_MINOR,
	.name				= "ibstrace",
	.fops				= &ibstrace_fops,
};


// Hack to resolve the address of some symbol via kprobes.
static u64 kprobe_resolve_sym(const char* name)
{
	int res;
	struct kprobe kp = { .symbol_name = name };
	u64 addr = 0;

	res = register_kprobe(&kp);
	if (res < 0)
		return 0;

	pr_info("ibstrace: found %s at %px\n", name, (void*)kp.addr);
	addr = (u64)kp.addr;

	unregister_kprobe(&kp);
	return addr;
}

// Jump into user code.
//
// WARNING: 
// This allows root the ability to execute arbitrary code in the kernel.
// Assume if you're using this, you know what you're doing.
void trampoline(void *info)
{
	int res;

	//int cpu = get_cpu();
	pr_info("ibstrace: trampoline CPU #%d\n", smp_processor_id());

	print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS, 16, 1, 
			code_buf, code_buf_len, 1);

	asm(
		"push %%rbx\n"
		"push %%rbp\n"
		"push %%r12\n"
		"push %%r13\n"
		"push %%r14\n"
		"push %%r15\n"
		"pushfq\n"

		"call *%%rax\n"

		"popfq\n"
		"pop %%r15\n"
		"pop %%r14\n"
		"pop %%r13\n"
		"pop %%r12\n"
		"pop %%rbp\n"
		"pop %%rbx\n"

		: "=a"(res)			// Input pointer in rax
		: "a"(code_buf)		// Output return value in rax
	);

	pr_info("ibstrace: trampoline returned %d\n", res);

	//put_cpu();

}

static __init int ibstrace_init(void)
{
	// We have to resolve these symbols in order to set pages as executable
	set_memory_x = (void*)kprobe_resolve_sym("set_memory_x");
	set_memory_nx = (void*)kprobe_resolve_sym("set_memory_nx");
	if ((set_memory_x == NULL) || (set_memory_nx == NULL)) {
		pr_err("ibstrace: couldn't resolve symbols\n");
		return -1;
	}

	// Register a character device
	if (misc_register(&ibstrace_dev) != 0) {
		pr_err("ibstrace: couldn't register device\n");
		return -1;
	}

	code_buf = vmalloc(CODE_BUFFER_SIZE);
	sample_buf = vmalloc(SAMPLE_BUFFER_SIZE);
	set_memory_x((unsigned long)code_buf, CODE_BUFFER_PAGES);

	pr_info("ibstrace: code_buf=%px (%ld)\n", code_buf, CODE_BUFFER_SIZE);
	pr_info("ibstrace: sample_buf=%px (%ld)\n", sample_buf, SAMPLE_BUFFER_SIZE);

#ifndef QEMU_BUILD
	// Initialize the local APIC for the target CPU
	smp_call_function_single(TARGET_CPU, ibs_apic_init, NULL, 1);
#endif

	return 0;
}

static __exit void ibstrace_exit(void)
{
	// Free allocations
	set_memory_nx((unsigned long)code_buf, CODE_BUFFER_PAGES);
	vfree(code_buf);
	vfree(sample_buf);

#ifndef QEMU_BUILD
	// Revert our APIC setup on the target CPU
	smp_call_function_single(TARGET_CPU, ibs_apic_exit, NULL, 1);
#endif 

	// Tear down character device
	misc_deregister(&ibstrace_dev);

	pr_info("ibstrace: unloaded module\n");
}

module_init(ibstrace_init);
module_exit(ibstrace_exit);
MODULE_LICENSE("GPL v2");
