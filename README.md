# ibstrace

`ibstrace` is a kernel module for measuring small pieces of user-submitted 
code with AMD IBS. `ibst` is a Rust library that you can use to interact
with the kernel module and parse samples.

For reference, I've tested/used this code on the following parts: 

- Ryzen 7 3950X ("Matisse", Zen 2)
- Ryzen 5 PRO 5650GE ("Cezanne", Zen 3)

Before using this, you should probably know that:

- **It's dangerous** (no guarantees that this actually works correctly)
- **It's dangerous** (lets you execute arbitrary code in the kernel)
- **It's dangerous** (totally unsafe by design, probably has subtle bugs too)

Otherwise, if you really do need to use this for some godforsaken reason, you
are most likely beyond help and know exactly what you're doing. 

