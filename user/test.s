.intel_syntax noprefix
_start:
	mov		rsi, 0x4000

_loop:
	mov		eax, 0x00000002
	nop
	cpuid
	nop
	nop
	sub		rsi, 1
	jne		_loop

_exit:
	mov		rax, 42
	ret

