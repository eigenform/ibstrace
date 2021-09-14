Kernel module and userspace tools for playing with AMD's "instruction-based 
sampling" (IBS) on my Ryzen 7 3950X.

Exposes a character device `/dev/ibstrace` which executes user-submitted chunks
of code in the context of the kernel and buffers up any IBS uop samples 
collected during execution.

**IMPORTANT:** There's little to no reason for you to use or even build this.
There are probably bugs. There are probably compatibility issues for other
AMD processors that aren't family 0x17 model 0x71. It's also *extremely unsafe*
by design, allowing you to execute arbitrary code in the kernel. 
Ideally, just avoid using this.

