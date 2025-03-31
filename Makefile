.PHONY: prod qemu user tests
all: options

options: 
	@echo "Try invoking make with one of the following options (where 'N' is the target core number):"
	@echo " 	make prod CORE=N - Build kernel module with Zen2-specific features"
	@echo " 	make qemu CORE=N - Build kernel module for testing w/ QEMU"

prod:
ifndef CORE
	$(error Must define target core, ie. 'make prod CORE=15')
endif
	@echo "# Building ibstrace kernel module ... "
	$(MAKE) -C ibstrace/ prod CORE=$(CORE)

qemu:
ifndef CORE
	$(error Must define target core, ie. 'make qemu CORE=15')
endif
	@echo "# Building ibstrace kernel module ... "
	$(MAKE) -C ibstrace/ qemu CORE=$(CORE)

clean:
	$(MAKE) -C ibstrace/ clean
