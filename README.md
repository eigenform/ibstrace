Exposes a character device `/dev/ibstrace` which executes user code in the
context of the kernel, and buffers up IBS samples collected during execution.
There's no reason for you to use or even build this.

**Important:** This lets you run untrusted code in kernel-mode.


# Building
Just run `make`.
