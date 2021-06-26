// SPDX-License-Identifier: GPL-2.0

#include <linux/smp.h>
#include <linux/fs.h>
#include <ibstrace.h>
#include "state.h"
#include "fops.h"

struct ibstrace_msg tmp;
extern void trampoline(void *info);
extern struct ibstrace_state state;

static call_single_data_t trampoline_csd = {
	.func = trampoline,
	.info = NULL,
};

long int ibstrace_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
{
	int res = 0;
	long idx;

	switch (cmd) {
	case IBSTRACE_CMD_WRITE:
		mutex_lock(&state.in_use);

		// NOTE: Does copy_from_user() validate this pointer?
		res = copy_from_user(&tmp, (struct ibstrace_msg *)arg, 
				sizeof(struct ibstrace_msg));
		if (res != 0) {
			res = -EINVAL;
			break;
		}
		if ((tmp.len > CODE_BUFFER_MAX_SIZE) || (tmp.len == 0)) {
			res = -EINVAL;
			break;
		}

		res = copy_from_user(state.code_buf, tmp.ptr, tmp.len);
		if (res != 0) {
			res = -EINVAL;
			break;
		}
		state.code_buf_len = tmp.len;
		pr_info("ibstrace: wrote %llu bytes\n", tmp.len);
		mutex_unlock(&state.in_use);
		break;

	case IBSTRACE_CMD_MEASURE:

		mutex_lock(&state.in_use);
		pr_info("ibstrace: dispatching code ...\n");
		smp_call_function_single_async(TARGET_CPU, &trampoline_csd);

		pr_info("ibstrace: waiting for lock ...\n");

		mutex_lock(&state.in_use);
		pr_info("ibstrace: trampoline completed?\n");
		mutex_unlock(&state.in_use);
		break;

	case IBSTRACE_CMD_READ:
		mutex_lock(&state.in_use);
		pr_info("ibstrace: dumping samples ...\n");

		long num_samples = atomic_long_read(&state.samples_collected);
		for (idx = 0; idx < num_samples; idx++) {
			pr_info("ibstrace: cpu=%d kernel=%d ctl=%016llx rip=%016llx data=%016llx\n",
					state.sample_buf[idx].cpu,
					state.sample_buf[idx].kernel,
					state.sample_buf[idx].op_ctl,
					state.sample_buf[idx].op_rip,
					state.sample_buf[idx].op_data
					);
		}

		pr_info("ibstrace: collected %lu samples\n", num_samples);

		atomic_long_xchg(&state.samples_collected, 0);
		mutex_unlock(&state.in_use);
		break;

	default:
		res = -EINVAL;
		break;
	}

	return res;
}


