/* SPDX-License-Identifier: GPL-2.0 */

#ifndef _FOPS_H
#define _FOPS_H

#include <linux/proc_fs.h>

#define SAMPLE_BUFFER_PAGES		256
#define SAMPLE_BUFFER_SIZE		(SAMPLE_BUFFER_PAGES * PAGE_SIZE)

#define CODE_BUFFER_PAGES		32
#define CODE_BUFFER_SIZE		(CODE_BUFFER_PAGES * PAGE_SIZE)


ssize_t ibstrace_write(struct file *file, const char __user *buf, 
		size_t count, loff_t *ppos);
ssize_t ibstrace_read(struct file *file, char __user *buf, 
		size_t count, loff_t *ppos);

#endif // _FOPS_H
