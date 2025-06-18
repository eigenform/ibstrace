// SPDX-License-Identifier: GPL-2.0

#ifndef _STATE_H
#define _STATE_H

#define CODE_BUFFER_PAGES		32
#define CODE_BUFFER_MAX_SIZE	(CODE_BUFFER_PAGES * PAGE_SIZE)

struct ibstrace_state {
	// Lock held when we're doing some operation
	struct mutex in_use;

	// The number of samples copied into the buffer
	atomic_long_t samples_collected;

	atomic_long_t precise_mode;

	// Pointer to buffer of samples
	struct sample *sample_buf;
	// Maximum number of samples 
	u64 sample_buf_capacity;
	// Length of the sample buffer in bytes
	u64 sample_buf_len;

	// Pointer to buffer with user code
	__u8 *code_buf;
	// Length of code buffer in bytes
	u64 code_buf_len;

	// A scratch page passed to user code
	struct page *__scratch_page;
	// Virtual address of the scratch page
	void *scratch_page;
	// Physical address of the scratch page
	unsigned long scratch_page_paddr;
};

#endif // _STATE_H
