Exposes a character device `/dev/ibstrace` which executes user code in the
context of the kernel, and buffers up IBS samples collected during execution.
There's no reason for you to use or even build this.

**IMPORTANT:** This lets you run arbitrary (potentially untrusted) code in 
kernel-mode just by writing to a character device. 

# Building
Just run `make`.
