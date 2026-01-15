# SPDX-License-Identifier: GPL-2.0

obj-m 			:= ibstrace.o
ibstrace-y 		:= trampoline_asm.o precise_trampoline_asm.o trampoline.o apic.o nmi.o fops.o main.o
KMOD_DIR 		:= /lib/modules/$(shell uname -r)/build
ccflags-y 	    := -g -I$(CURDIR)/../include -Wundef -DTARGET_CPU=$(CORE)
asflags-y 	    := -I$(CURDIR)/../include/asm 

prod:
	$(MAKE) V=1 -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)"
clean:
	$(MAKE) V=1 -C $(KMOD_DIR) M=$(CURDIR) clean
