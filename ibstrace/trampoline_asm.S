// SPDX-License-Identifier: GPL-2.0

#include <linux/linkage.h>
#incldue <asm/delay.h>

.intel_syntax noprefix

// Trampoline into the buffer filled with user code.
// RDI - address to user code
// RSI - virtual address of some scratch page
//
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
	//	- Set IBS_OP_CNT_CTL (19), IBS_OP_EN (17), and IBS_OP_MAX_CNT ([15:0])
	//	- Write back IBS_OP_CTL to enable sampling
	//
	// NOTE: The low 4 bits in the op counter are hardware-randomized.
	// IBS_OP_MAX_CNT actually maps to bits [19:4] in the counter.
	// The value set here has to be large enough such that, by the time the
	// counter overflows and tags a micro-op, we've entered the user's code.
	//
	// [More] importantly, the same constraint also applies for the return 
	// path from the NMI handler, which is probably longer than the path from
	// this 'wrmsr' to user code. 
	//
	// The only actual solution to these worries involves *not* implementing 
	// this as a Linux kernel module (and instead, having this entire scheme
	// as a purpose-built kernel where the path between IBS NMIs and measured 
	// code is as short as possible).

	mov		ecx, 0xc0011033
	rdmsr   
	or		eax, 0x000a0100
	wrmsr
#endif // QEMU_BUILD

	// Call into user code (with some argument in RDI).
	mov		rax, rdi
	mov		rdi, rsi
	call	rax

#ifndef QEMU_BUILD
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

	mov		ecx, 0xc0011033
	mov		eax, 0x00040000
	xor		edx, edx
	wrmsr

	// Wait in a busy loop for a hanging NMI to be dispatched and handled.
	// You'll know this doesn't work if dmesg tells you about unexpected NMIs.
	// We probably don't *need* to call __udelay() here (you could just write
	// the loop in here), but whatever.

	mov		rdi, 0x1
	call	__udelay

	// Finally, zero out all the bits in IBS_OP_CTL.
	mov		ecx, 0xc0011033
	xor		eax, eax
	xor		edx, edx
	wrmsr

	// Restore the return value from user code
	pop		rax
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

