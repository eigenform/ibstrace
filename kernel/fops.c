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

ssize_t ibstrace_read(struct file *file, char __user *buf, size_t count,
		loff_t *fpos)
{
	ssize_t res;
	int num_bytes;
	long num_samples;

	mutex_lock(&state.in_use);

	num_samples = atomic_long_read(&state.samples_collected);
	num_bytes = (num_samples * sizeof(struct sample));

	if ((count == 0) || (num_samples == 0)) {
		res = 0;
		goto out;
	} 
	if (count > num_bytes) {
		count = num_bytes;
	}

	// Consume the buffer
	if (copy_to_user(buf, state.sample_buf, count)) {
		res = -EFAULT;
		goto out;
	}
	atomic_long_xchg(&state.samples_collected, 0);
	memset(state.sample_buf, 0, num_bytes);
	res = count;

out:
	mutex_unlock(&state.in_use);
	return res;
}

long int ibstrace_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
{
	long int res = 0;
	long num_samples;

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
		//pr_info("ibstrace: wrote %llu bytes\n", tmp.len);
		mutex_unlock(&state.in_use);
		break;

	case IBSTRACE_CMD_MEASURE:
		mutex_lock(&state.in_use);
		smp_call_function_single_async(TARGET_CPU, &trampoline_csd);

		// Wait around until the trampoline returns and the target core
		// releases the lock. There's probably a better way to do this ...
		mutex_lock(&state.in_use);
		mutex_unlock(&state.in_use);
		break;

	case IBSTRACE_CMD_SAMPLES:
		mutex_lock(&state.in_use);
		num_samples = atomic_long_read(&state.samples_collected);
		res = num_samples;
		mutex_unlock(&state.in_use);
		break;

	case IBSTRACE_CMD_CAPACITY:
		mutex_lock(&state.in_use);
		res = IBSTRACE_SAMPLE_CAPACITY;
		mutex_unlock(&state.in_use);
		break;

	default:
		res = -EINVAL;
		break;
	}

	return res;
}

