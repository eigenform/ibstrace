.PHONY: prod qemu user tests
all: options

options: 
	@echo "Try invoking make with one of the following options:"
	@echo " 	make prod - Build kernel module with Zen2-specific features"
	@echo " 	make qemu - Build kernel module for testing w/ QEMU"
	@echo " 	make user - Build userspace tools"

prod:
	$(MAKE) -C kernel/ prod
qemu:
	$(MAKE) -C kernel/ qemu
user:
	$(MAKE) -C user/
test:
	$(MAKE) -C test/

clean:
	$(MAKE) -C kernel/ clean
	$(MAKE) -C user/ clean
	$(MAKE) -C test/ clean
