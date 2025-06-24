// SPDX-License-Identifier: GPL-2.0

#include <linux/linkage.h>
#include "msr.h"

#define MAX_CNT		0x00100000
#define CUR_CNT_MASK	(MAX_CNT - 1)

.intel_syntax noprefix

// Trampoline into the buffer filled with user code.
// RDI - pointer to user code (to be executed)
// RSI - Argument passed through to user code in RDI
// RDX - offset to target micro-op

SYM_FUNC_START(__precise_trampoline_start)

	push	rbx
	push	rbp
	push	r12
	push	r13
	push	r14
	push	r15
	pushfq

	// Pointer to user code
	mov		r15, rdi

	// Argument passed through to user code in RDI
	mov		rdi, rsi
	xor		rsi, rsi

	// Mask off any high bits in the offset
	mov		rbx, rdx
	and		rbx, CUR_CNT_MASK

	// Input to WRMSR for enabling IBS in op-sampling mode, where: 
	//	- MAX_CNT is set to 0x00100000
	//	- CUR_CNT is set to (MAX_CNT - RBX)
	mov		ecx, IBS_OP_CTL
	mov		eax, 0x001a0000
	mov		edx, MAX_CNT
	sub		edx, ebx
	xor		rbx, rbx

	// Enable IBS, then immediately call into user code. 
	// NOTE: WRSMR *should* already be serializing. 
	// NOTE: In order to actually recover a trace, it's important that the path 
	// between WRMSR and user code is very short and ideally *deterministic*.
	// In our case, the indirect CALL to user code is necessary and sufficient.
SYM_INNER_LABEL(__precise_trampoline_wrmsr, SYM_L_GLOBAL)
	wrmsr 

SYM_INNER_LABEL(__precise_trampoline_call, SYM_L_GLOBAL)
	call	r15
	lfence

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

