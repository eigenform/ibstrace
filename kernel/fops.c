
#include "fops.h"

ssize_t ibstrace_write(struct file *file, const char __user *buf, 
		size_t count, loff_t *ppos)
{
	pr_info("ibstrace: write\n");
	return -1;
}
ssize_t ibstrace_read(struct file *file, char __user *buf, 
		size_t count, loff_t *ppos)
{
	pr_info("ibstrace: read\n");
	return 0;
}

