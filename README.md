
`ibstrace` is a kernel module for measuring small pieces of user-submitted 
code with AMD IBS. `ibst-rs` is a library that you can use to interact
with the kernel module and parse samples.

I've written this specifically for the Ryzen 7 3950X in my desktop machine, 
for learning about/experimenting with the Zen 2 microarchitecture. You should 
probably know that:

- **It's dangerous** (no compatibility outside of AMD Family 17h, Model 71h)
- **It's dangerous** (lets you execute arbitrary code in the kernel)
- **It's dangerous** (mostly unsafe by design, probably has bugs too)

That said, if you *really* do need to use this for some godforsaken reason, 
I'm going to assume that you are beyond help, or that you know exactly what 
you're doing. 

