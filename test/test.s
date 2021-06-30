.intel_syntax noprefix

# NOTE: The AMD Architecture Programmer's Manual talks a little bit about edge
# cases where tagged uops do not retire:
#
#	"Aborted ops do not produce an IBS execution sample. If the tagged op
#	aborts (i.e., does not retire), hardware resets bits 26:7 of the op
#	interval counter to zero, and bits 6:0 to a random value. The op counter
#	continues to increment and another op is selected when the value in bits
#	26:4 of the op interval counter equals the value in the IbsOpMaxCnt field"
#
# Regardless, if we have code where we expect an exception to occur, we need
# to emit some extra data at the start of this binary in order to help the
# kernel module resolve the address of the faulting instruction at runtime.
# Specifically (a) the offset to the potentially-faulting instruction, and (b)
# the offset where execution should resume after an exception is handled.

# Offset from the start of this binary to the faulting instruction.
#.global fault_instr
#fault_from
#	.long (_fault_instr - .)


_start:
	mov		rsi, 0x4000
loop_top:
	mov		ecx, 0x00000010
	rdmsr
	sub		rsi, 1
	jne		loop_top

_exit:
	mov		rax, 42
	ret

