#ifndef _IBSTRACE_H
#define _IBSTRACE_H

#include <linux/types.h>

#define TARGET_CPU				4
#define IBSTRACE_CMD_WRITE		1

struct ibstrace_msg {
	void *ptr;
	__u64 len;
};

struct sample {
	__u64 foo;
	__u64 bar;
};

struct ibstrace_state {
	struct sample *sample_buffer;
	__u8 *user_code;
};

#endif // _IBSTRACE_H
