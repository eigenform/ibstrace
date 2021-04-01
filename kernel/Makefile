
obj-m 		:= ibstrace.o
ibstrace-y 	:= apic.o nmi.o fops.o main.o
KMOD_DIR 	:= /lib/modules/$(shell uname -r)/build

CFLAGS 		+= -g -I$(CURDIR)/

all:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) CFLAGS="$(CFLAGS)"
clean:
	$(MAKE) -C $(KMOD_DIR) M=$(CURDIR) clean
