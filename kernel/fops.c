// SPDX-License-Identifier: GPL-2.0

#include "fops.h"

extern u8 *code_buf;

ssize_t ibstrace_write(struct file *file, const char __user *buf, 
		size_t count, loff_t *ppos)
{
	pr_info("ibstrace: write (cpu #%d)\n", smp_processor_id());
	copy_from_user(code_buf, buf, count);
	print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS, 16, 1, code_buf, count, 1);
	return 0;
}
ssize_t ibstrace_read(struct file *file, char __user *buf, 
		size_t count, loff_t *ppos)
{
	pr_info("ibstrace: read (cpu #%d)\n", smp_processor_id());
	return 0;
}


