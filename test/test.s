.intel_syntax noprefix
_start:
	mov		rsi, 0x4000

_loop:
	mov		ecx, 0x00000010
	rdmsr
	sub		rsi, 1
	jne		_loop

_exit:
	mov		rax, 42
	ret

