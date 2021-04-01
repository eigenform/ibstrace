/* SPDX-License-Identifier: GPL-2.0 */

#ifndef _FOPS_H
#define _FOPS_H

#include <linux/proc_fs.h>

ssize_t ibstrace_write(struct file *file, const char __user *buf, 
		size_t count, loff_t *ppos);
ssize_t ibstrace_read(struct file *file, char __user *buf, 
		size_t count, loff_t *ppos);

#endif // _FOPS_H
