// trampoline_example.c
//
// This is just a little model of what we want to happen in the kernel, since 
// it's a lot easier to just debug things in userspace first.

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>

uint8_t *code_buf;
uint8_t mov42[8] = { 
	0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00,	// mov rax, 42
	0xc3										// ret
};

int main(int argc, char *argv[]) {
	int res;

	code_buf = mmap(0, 4096, PROT_READ | PROT_WRITE | PROT_EXEC,
			MAP_ANONYMOUS | MAP_PRIVATE, 0, 0);
	memcpy(code_buf, mov42, 8);
	printf("Jumping to %p ...\n", code_buf);

	asm(
		"push %%rbx\n"
		"push %%rbp\n"
		"push %%r12\n"
		"push %%r13\n"
		"push %%r14\n"
		"push %%r15\n"
		"pushfq\n"

		"call *%%rax\n"

		"popfq\n"
		"pop %%r15\n"
		"pop %%r14\n"
		"pop %%r13\n"
		"pop %%r12\n"
		"pop %%rbp\n"
		"pop %%rbx\n"

		: "=a"(res)			// Input pointer in rax
		: "a"(code_buf)		// Output return value in rax
	);

	munmap(code_buf, 4096);
	return res;
}
