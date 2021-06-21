#!/bin/bash

# I'm using an Ubuntu20 machine for this, FYI.
# You can do something like this to add `-s` to virt-manager's qemu invocation:
#
#	virt-xml -c qemu:///system <domain> \
#		--edit --confirm \
#		--qemu-commandline='-s'
#
# For Ubuntu, you're going to want dbgsym packages, see:
#
#	https://wiki.ubuntu.com/Debug%20Symbol%20Packages
#	https://wiki.ubuntu.com/Kernel/Dev/KernelGitGuide
#
# You can just install these on the guest and pull /usr/lib/debug/ onto the
# host machine (it's uhhh like 7GB or something

# Copy the kernel image from the VM
if [ ! -e /tmp/vmlinux-5.8.0-48-generic ]; then
	scp meta@192.168.100.182:/usr/lib/debug/boot/vmlinux-5.8.0-48-generic /tmp/
fi

gdb -ex 'file /tmp/vmlinux-5.8.0-48-generic' \
	-ex 'set substitute-path ~/src/ubuntu-focal/ .'
	-ex 'add-auto-load-safe-path ~/src/ubuntu-focal/'
	-ex 'target remote :1234'

