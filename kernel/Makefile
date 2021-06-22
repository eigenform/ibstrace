# SPDX-License-Identifier: GPL-2.0

obj-m 		:= ibstrace.o
ibstrace-y 	:= apic.o nmi.o fops.o main.o
KMOD_DIR 	:= /lib/modules/$(shell uname -r)/build

CFLAGS 		+= -g -I$(CURDIR)/

prod:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)"
qemu:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)" \
		EXTRA_CFLAGS="-DQEMU_BUILD=1"
clean:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) clean
