.PHONY: prod qemu user tests
all: options

options: 
	@echo "Try invoking make with one of the following options:"
	@echo " 	make prod - Build kernel module with Zen2-specific features"
	@echo " 	make qemu - Build kernel module for testing w/ QEMU"

prod:
	@echo "# Building ibstrace kernel module ..."
	$(MAKE) -C ibstrace/ prod
qemu:
	$(MAKE) -C ibstrace/ qemu
clean:
	$(MAKE) -C ibstrace/ clean
