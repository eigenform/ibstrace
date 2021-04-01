#ifndef _IBSTRACE_H
#define _IBSTRACE_H

struct sample {
	u64 foo;
	u64 bar;
};

struct ibstrace_state {
	struct sample *sample_buffer;
	u8 *user_code;
};



#endif // _IBSTRACE_H
