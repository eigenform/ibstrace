#ifndef _IBSTRACE_H
#define _IBSTRACE_H

#include <linux/types.h>
#include <linux/sched.h>

// The target CPU used to execute and sample some code
#define TARGET_CPU					0

// The maximum capacity of the sample buffer (in number of samples)
#define IBSTRACE_SAMPLE_CAPACITY	0x4000

// ioctl() command: submit code to-be-measured
#define IBSTRACE_CMD_WRITE			0x00001000
// ioctl() command: execute and sample code
#define IBSTRACE_CMD_MEASURE		0x00002000
// ioctl() command: return the number of collected samples
#define IBSTRACE_CMD_SAMPLES		0x00004000
// ioctl() command: return the maximum sample buffer capacity
#define IBSTRACE_CMD_CAPACITY		0x00008000

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
	__u64 dc_lin_addr;
	__u64 dc_phys_addr;
};

#endif // _IBSTRACE_H
