// SPDX-License-Identifier: GPL-2.0

#ifndef _FOPS_H
#define _FOPS_H

long int ibstrace_ioctl(struct file *file, unsigned int cmd, unsigned long arg);
ssize_t ibstrace_read(struct file *file, char __user *buf, size_t count,
		loff_t *fpos);

#endif // _FOPS_H
