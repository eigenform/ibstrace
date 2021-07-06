// SPDX-License-Identifier: GPL-2.0

#include <linux/linkage.h>
.intel_syntax noprefix

// Trampoline into the buffer filled with user code (an address in RDI).
// It's easier just to write this in assembly so we can guarantee that our
// sampling begins and ends as close to user code as possible.

SYM_FUNC_START(__trampoline_start)

	push	rbx
	push	rbp
	push	r12
	push	r13
	push	r14
	push	r15
	pushfq

#ifndef QEMU_BUILD
	// Enable sampling with the following operations:
	//
	//	- Read IBS_OP_CTL into EDX:EAX
	//	- Set IBS_OP_CNT_CTL (19), IBS_OP_EN (17), and IBS_OP_MAX_CNT
	//	- Write back IBS_OP_CTL to enable sampling

	mov		ecx, 0xc0011033
	rdmsr   
	or		eax, 0xa0100
	wrmsr
#endif // QEMU_BUILD


	// Indirectly branch (return value in RAX)
	call	rdi


#ifndef QEMU_BUILD
	// Save the return value in RSI (we need RAX to deal with MSRs)
	mov		rdi, rax 

	// Clear all bits in IBS_OP_CTL except for IBS_OP_VAL (18).
	//
	// There are apparently corner cases where an NMI may be dispatched some
	// time *after* we clear the IBS_OP_EN (17) bit. Since our NMI handler 
	// expects IBS_OP_VAL (18) to be set, we need to keep it set until the
	// interrupt handler clears it. If IBS_OP_VAL is set when handling an
	// NMI, let the IBS_OP_MAX_CNT bits (zeroed out here) indicate that the
	// last sample should be ignored.

	mov		ecx, 0xc0011033
	mov		eax, 0x00400000
	xor		edx, edx
	wrmsr

	// Wait in a busy loop for a hanging NMI to be dispatched and handled.
	// I have no idea how much time we need to wait, so for all I know, this
	// might do absolutely nothing to solve this :^)

	mov		rax, 0x01000000
1:	dec		rax
	jnz		1b

	// Finally, zero out all the bits in IBS_OP_CTL and move the return
	// value back from RDI to RAX.

	mov		ecx, 0xc0011033
	xor		eax, eax
	xor		edx, edx
	wrmsr
	mov		rax, rdi
#endif // QEMU_BUILD

	popfq
	pop		r15
	pop		r14
	pop		r13
	pop		r12
	pop		rbp
	pop		rbx
	ret

SYM_FUNC_END(__trampoline_start)

