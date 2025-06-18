# SPDX-License-Identifier: GPL-2.0

obj-m 			:= ibstrace.o
ibstrace-y 		:= trampoline_asm.o precise_trampoline_asm.o trampoline.o apic.o nmi.o fops.o main.o
KMOD_DIR 		:= /lib/modules/$(shell uname -r)/build
EXTRA_CFLAGS 	:= -g -I$(CURDIR)/../include -Wundef -DTARGET_CPU=$(CORE)

prod:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)" \
		EXTRA_CFLAGS="$(EXTRA_CFLAGS)"
clean:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) clean
