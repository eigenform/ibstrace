// SPDX-License-Identifier: GPL-2.0

#include <linux/linkage.h>
#include "msr.h"

.extern __udelay

.intel_syntax noprefix

// Trampoline into the buffer filled with user code.
// RDI - address to user code (to be executed)
// RSI - virtual address of some scratch page, passed to user code as RDI
// RDX - offset to target micro-op

SYM_FUNC_START(__precise_trampoline_start)

	push	rbx
	push	rbp
	push	r12
	push	r13
	push	r14
	push	r15
	pushfq

	mov		rbx, rdx
	and     rbx, 0x000fffff

	// Enable IBS in op-sampling mode, max_cnt=0x00100000, cur_cnt=0x00100000-RBX
	// NOTE: It seems like the last 9 uops in WRMSR can be sampled 
	// (corresponding offset 0-8)?

	mov		ecx, IBS_OP_CTL
	mov     eax, 0x001a0000
	mov     edx, 0x00100000
	sub     edx, ebx
	wrmsr 

	// Call into user code (with some argument in RDI).
	mov		rax, rdi // (offset 9 and 10)?
	mov		rdi, rsi // (offset 11?)
	call	rax      // (offset 12 and 13)?

	// We might end up clobbering RAX before we return from the trampoline, 
	// so save the return value from our measured code on the stack.
	push	rax

	// Clear all bits in IBS_OP_CTL except for IBS_OP_VAL (18).
	//
	// There are apparently corner cases where an NMI may be dispatched some
	// time *after* we clear the IBS_OP_EN (17) bit. Since our NMI handler 
	// expects IBS_OP_VAL (18) to be set, we need to keep it set until the
	// interrupt handler clears it. If IBS_OP_VAL is set when handling an
	// NMI, let the IBS_OP_MAX_CNT bits (zeroed out here) indicate that the
	// last sample should be ignored.

	mov		ecx, IBS_OP_CTL
	mov		eax, 0x00040000
	xor		edx, edx
	wrmsr

	// Waste some time while we wait for the NMI to be handled 
	mov		rdi, 0x1000
.Lloop:
	dec		rdi
	cmp		rdi, 0
	jne		.Lloop

	// Finally, zero out all the bits in IBS_OP_CTL.
	mov		ecx, IBS_OP_CTL
	xor		eax, eax
	xor		edx, edx
	wrmsr

	// Restore the return value from user code
	pop		rax

	popfq
	pop		r15
	pop		r14
	pop		r13
	pop		r12
	pop		rbp
	pop		rbx
	ret

SYM_FUNC_END(__precise_trampoline_start)

