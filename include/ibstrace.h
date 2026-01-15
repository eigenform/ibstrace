#ifndef _IBSTRACE_H
#define _IBSTRACE_H

#include <linux/types.h>
#include <linux/sched.h>
#include "asm/ibstrace_asm.h"

// The maximum capacity of the sample buffer (in number of samples)
#define IBSTRACE_SAMPLE_CAPACITY	0x40000

// ioctl() command: submit code to-be-measured
#define IBSTRACE_CMD_WRITE			0x00001000

// ioctl() command: execute and sample user code
#define IBSTRACE_CMD_MEASURE		0x00002000

// ioctl() command: return the number of collected samples
#define IBSTRACE_CMD_SAMPLES		0x00004000

// ioctl() command: return the maximum sample buffer capacity
#define IBSTRACE_CMD_CAPACITY		0x00008000

// ioctl() command: execute and sample a particular op in user code
#define IBSTRACE_CMD_PRECISE		0x00020000

// Arguments passed to IBSTRACE_CMD_WRITE 
struct ibstrace_msg {
	// Pointer to a buffer with user code to-be-uploaded
	void *ptr;
	// Buffer length
	__u64 len;
};

// Arguments passed to IBSTRACE_CMD_PRECISE
struct ibstrace_precise_msg {
	// Arbitrary user input, passed to measured code in RDI
	void *ptr;
	// Target sample offset (in dispatched micro-ops)
	__u64 offset;
};

// IBS sample data
struct sample {
	__u64 op_ctl;
	__u64 op_rip;
	__u64 op_data;
	__u64 op_data2;
	__u64 op_data3;
	__u64 dc_lin_addr;
	__u64 dc_phys_addr;
	__u64 tgt_rip;
};

#endif // _IBSTRACE_H
