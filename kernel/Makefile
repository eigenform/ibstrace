# SPDX-License-Identifier: GPL-2.0

obj-m 			:= ibstrace.o
ibstrace-y 		:= trampoline.o apic.o nmi.o fops.o main.o
KMOD_DIR 		:= /lib/modules/$(shell uname -r)/build
EXTRA_CFLAGS 	:= -g -I$(CURDIR)/../include

prod:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)" \
		EXTRA_CFLAGS="$(EXTRA_CFLAGS)"
qemu:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)" \
		EXTRA_CFLAGS="$(EXTRA_CFLAGS) -DQEMU_BUILD=1"
clean:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) clean
