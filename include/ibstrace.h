// SPDX-License-Identifier: GPL-2.0

#ifndef _IBSTRACE_H
#define _IBSTRACE_H

#include <linux/types.h>
#include <linux/sched.h>

#define TARGET_CPU				4

#define IBSTRACE_CMD_WRITE		0x1000
#define IBSTRACE_CMD_MEASURE	0x2000
#define IBSTRACE_CMD_READ		0x4000


struct ibstrace_msg {
	void *ptr;
	__u64 len;
};

struct sample {
	__u64 op_ctl;
	__u64 op_rip;
	__u64 op_data;
	__u64 op_data2;
	__u64 op_data3;
	__u64 op_data4;
	__u64 dc_lin_addr;
	__u64 dc_phys_addr;
	pid_t tid;
	pid_t pid;
	int	  cpu;
	int   kernel;
};

#endif // _IBSTRACE_H
