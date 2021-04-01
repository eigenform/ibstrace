// SPDX-License-Identifier: GPL-2.0

#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/slab.h>

#include "ibstrace.h"
#include "apic.h"
#include "fops.h"

#include <linux/kallsyms.h>
#include <linux/kprobes.h>

static int (*set_memory_x)(unsigned long, int) = NULL;
static int (*set_memory_nx)(unsigned long, int) = NULL;
static unsigned long find_symbol(const char* name)
{
	int res;
	struct kprobe kp = { .symbol_name = name };

	res = register_kprobe(&kp);
	if (res < 0)
		return 0;

	unregister_kprobe(&kp);
	pr_info("ibstrace: found %s at %p\n", name, (void*)kp.addr);
	return (unsigned long)kp.addr;
}

#define TARGET_CPU		4
#define SAMPLE_BUFFER_SIZE	(4 * 1024 * 1024)
#define CODE_BUFFER_SIZE	(1 * 1024 * 1024)

static struct proc_dir_entry *procfs_entry;
static const struct proc_ops ibstrace_fops = {
	.proc_write	= ibstrace_write,
	.proc_read	= ibstrace_read,
};

struct ibstrace_state global_state = {
	.sample_buffer	= NULL,
	.user_code	= NULL,
};


static void trampoline(void)
{
}

static __init int ibstrace_init(void)
{
	int err;
	set_memory_x = (void*)find_symbol("set_memory_x");
	set_memory_nx = (void*)find_symbol("set_memory_nx");

	// Allocate for user code, poison with INT3, mark executable
	global_state.user_code = kmalloc(1024, GFP_KERNEL);
	memset(global_state.user_code, 0xcc, 1024);
	set_memory_x(global_state.user_code, 1024 / PAGE_SIZE);

	global_state.sample_buffer = kmalloc(1024 * 1024, GFP_KERNEL);


	// Initialize the local APIC for the target CPU
	smp_call_function_single(TARGET_CPU, ibs_apic_init, NULL, 1);

	// Create a procfs entry
	procfs_entry = proc_create("ibstrace", 0, NULL, &ibstrace_fops);
	if (!procfs_entry) {
		pr_err("ibstrace: couldn't create procfs entry\n");
		goto ng_apic;
	}

ok:
	return 0;
ng_apic:
	smp_call_function_single(TARGET_CPU, ibs_apic_exit, NULL, 1);
	return -1;
}

static __exit void ibstrace_exit(void)
{
	set_memory_nx(global_state.user_code, 1024 / PAGE_SIZE);
	kfree(global_state.user_code);
	kfree(global_state.sample_buffer);
	smp_call_function_single(TARGET_CPU, ibs_apic_exit, NULL, 1);
	proc_remove(procfs_entry);
	pr_info("ibstrace: unloaded module\n");
}

module_init(ibstrace_init);
module_exit(ibstrace_exit);
MODULE_LICENSE("GPL v2");
