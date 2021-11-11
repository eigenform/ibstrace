Kernel module and userspace tools for playing with AMD's "instruction-based 
sampling" (IBS) on my Ryzen 7 3950X.

Exposes a character device `/dev/ibstrace` which executes user-submitted chunks
of code in the context of the kernel and buffers up the IBS uop samples 
collected during execution.

## Usage

**IMPORTANT:** There's little to no reason for you to use or even build this.
There are probably bugs. There are probably compatibility issues for other
AMD processors that aren't family 0x17 model 0x71. It's also *extremely unsafe*
by design, allowing you to execute arbitrary code in the kernel. 
Ideally, just avoid using this.

If you *really* do need to run this for some godforsaken reason, I'm going to 
assume that you are beyond help, or that you know exactly what you're doing. 
For reference, the way I typically use it is like:

```
# I've been disabling SMT while running this, but I don't think it's actually
# necessary considering that the IBS MSRs are duplicated per-thread.

$ sudo ./scripts/set-ht off

# Load the kernel module and use tools from ibst-rs.
# I'm usually watching `dmesg` to make sure my machine doesn't implode.

$ sudo insmod kernel/ibstrace.ko
$ cd ibst-rs/  
$ cargo build --release --bin ibst-cpuid
$ sudo ./target/release/ibst-cpuid
...

# Unload the kernel module and restore SMT for all cores.

$ sudo rmmod ibstrace
$ sudo ./scripts/set-ht on
```

