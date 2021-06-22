// SPDX-License-Identifier: GPL-2.0

#include "fops.h"
#include <ibstrace.h>

extern u8 *code_buf;
extern u64 code_buf_len;
struct ibstrace_msg tmp;
extern void trampoline(void *info);

long int ibstrace_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
{
	// NOTE: You need to probably guard this with a mutex

	int res;
	switch (cmd) {

	case IBSTRACE_CMD_WRITE:
		// Copy a message (I assume that copy_from_user() at least does some
		// simple validation to make sure this pointer isn't garbage?)
		res = copy_from_user(&tmp, (struct ibstrace_msg *)arg, sizeof(struct ibstrace_msg));
		if (res != 0) {
			pr_err("ibstrace: couldn't copy message\n");
			return -EINVAL;
		}

		print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS, 16, 8, 
				&tmp, sizeof(struct ibstrace_msg), 1);

		// Don't handle user data larger than the maximum size
		if ((tmp.len > CODE_BUFFER_SIZE) || (tmp.len == 0)) {
			pr_err("ibstrace: invalid buffer length\n");
			return -EINVAL;
		}

		// Copy the actual data into the executable buffer
		res = copy_from_user(code_buf, tmp.ptr, tmp.len);
		if (res != 0) {
			pr_err("ibstrace: couldn't copy buffer\n");
			return -EINVAL;
		}
		code_buf_len = tmp.len;

		print_hex_dump(KERN_INFO, "", DUMP_PREFIX_ADDRESS, 16, 1, 
				code_buf, code_buf_len, 1);
		break;

	case IBSTRACE_CMD_MEASURE:
		smp_call_function_single(TARGET_CPU, trampoline, NULL, 1);
		break;

	default:
		return -EINVAL;
	}

	return 0;
}


