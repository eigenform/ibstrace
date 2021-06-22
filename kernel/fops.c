// SPDX-License-Identifier: GPL-2.0

#include "fops.h"

extern u8 *code_buf;

ssize_t ibstrace_write(struct file *file, const char __user *buf, 
		size_t count, loff_t *ppos)
{
	int res;
	pr_info("ibstrace: write from cpu #%d\n", smp_processor_id());

	if (count > CODE_BUFFER_SIZE) {
		pr_err("ibstrace: input too large (%ld)\n", count);
		return -1;
	}

	res = copy_from_user(code_buf, buf, count);
	if (res != 0) {
		pr_err("ibstrace: copy_from_user() returned non-zero (%d)\n", res);
		return -1;
	}

	print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS, 16, 1, code_buf, count, 1);
	return 0;
}
ssize_t ibstrace_read(struct file *file, char __user *buf, 
		size_t count, loff_t *ppos)
{
	pr_info("ibstrace: read from cpu #%d\n", smp_processor_id());
	return 0;
}


