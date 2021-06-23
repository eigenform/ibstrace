.intel_syntax noprefix
_start:
	mov		rax, 0x1000

_loop:
	xor		rcx, rcx
	sub		rax, 1
	jne		_loop
	ret

