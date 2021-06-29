// SPDX-License-Identifier: GPL-2.0

#include <linux/module.h>
#include <linux/slab.h>
#include <linux/vmalloc.h>
#include <linux/nmi.h>

#include <linux/kallsyms.h>
#include <linux/kprobes.h>

#include <linux/device.h>
#include <linux/miscdevice.h>

#include <ibstrace.h>
#include "state.h"
#include "apic.h"
#include "fops.h"
#include "nmi.h"


extern void trampoline(void *info);

// Shared state associated with this module
struct ibstrace_state state = {
	.sample_buf = NULL,
	.sample_buf_capacity = IBSTRACE_SAMPLE_CAPACITY,
	.sample_buf_len = sizeof(struct sample) * IBSTRACE_SAMPLE_CAPACITY,
	.samples_collected = ATOMIC_INIT(0),
};


static int (*set_memory_x)(unsigned long, int) = NULL;
static int (*set_memory_nx)(unsigned long, int) = NULL;

// File operations for the character device
static const struct file_operations ibstrace_fops = {
	.owner				= THIS_MODULE,
	.unlocked_ioctl		= ibstrace_ioctl,
	.read				= ibstrace_read,
};

// The character device exposed by this module
static struct miscdevice ibstrace_dev = {
	.minor				= MISC_DYNAMIC_MINOR,
	.name				= "ibstrace",
	.fops				= &ibstrace_fops,
};


// Cursed hack #1: 
// This resolves the address of some symbol during runtime via kprobes. 
// Don't be suprised if this breaks in future kernels.
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

//// Cursed hack #2: 
//// You might want to use this module to measure some snippit of code which
//// intentionally causes an exception.  
////
//static struct exception_table_entry extable = {
//	.insn = 0,
//	.fixup = 0,
//	.handler = 0,
//};

static __init int ibstrace_init(void)
{
	int err;

	//THIS_MODULE->extable = &extable;

#ifndef QEMU_BUILD
	// Avoid initializing this module if IBS isn't supported on this machine
	struct cpuinfo_x86 *info = &boot_cpu_data;
	if (!(info->x86_vendor == X86_VENDOR_AMD)) {
		pr_err("ibstrace: unsupported CPU\n");
		return -1;
	}
	if (!boot_cpu_has(X86_FEATURE_IBS)) {
		pr_err("ibstrace: cpuid says IBS isn't supported?\n");
		return -1;
	}
#endif

	// We have to resolve these symbols in order to set pages as executable.
	// I wonder if there's another way to do this?
	set_memory_x = (void*)kprobe_resolve_sym("set_memory_x");
	set_memory_nx = (void*)kprobe_resolve_sym("set_memory_nx");
	if ((set_memory_x == NULL) || (set_memory_nx == NULL)) {
		pr_err("ibstrace: couldn't resolve symbols\n");
		return -1;
	}

	// Register a character device at /dev/ibstrace
	if (misc_register(&ibstrace_dev) != 0) {
		pr_err("ibstrace: couldn't register device\n");
		return -1;
	}

#ifndef QEMU_BUILD
	// Initialize the local APIC for the target CPU
	smp_call_function_single(TARGET_CPU, ibs_apic_init, NULL, 1);
	err = register_nmi_handler(NMI_LOCAL, ibs_nmi_handler,
			NMI_FLAG_FIRST, "ibstrace");
	if (err) {
		pr_err("ibstrace: register_nmi_handler() returned %d\n", err);
		return -1;
	}
#endif

	// Allocate space for sample data, and for user input.
	mutex_init(&state.in_use);
	state.code_buf = vmalloc(CODE_BUFFER_MAX_SIZE);
	state.sample_buf = vmalloc(state.sample_buf_len);
	set_memory_x((unsigned long)state.code_buf, CODE_BUFFER_PAGES);
	pr_info("ibstrace: code_buf=%px\n", state.code_buf);
	pr_info("ibstrace: sample_buf=%px\n", state.sample_buf);
	pr_info("ibstrace: trampoline=%px\n", trampoline);

	return 0;
}

static __exit void ibstrace_exit(void)
{
	set_memory_nx((unsigned long)state.code_buf, CODE_BUFFER_PAGES);
	vfree(state.code_buf);
	vfree(state.sample_buf);

#ifndef QEMU_BUILD
	smp_call_function_single(TARGET_CPU, ibs_apic_exit, NULL, 1);
	unregister_nmi_handler(NMI_LOCAL, "ibstrace");
#endif 

	misc_deregister(&ibstrace_dev);
	pr_info("ibstrace: unloaded module\n");
}

module_init(ibstrace_init);
module_exit(ibstrace_exit);
MODULE_LICENSE("GPL v2");
