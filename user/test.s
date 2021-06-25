.intel_syntax noprefix
_start:
	mov		rsi, 0x1000

_loop:
	xor		rcx, rcx
	xor		rdx, rdx
	sub		rsi, 1
	jne		_loop

_exit:
	mov		rax, 42
	ret

