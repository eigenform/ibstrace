.PHONY: prod clean
all: options

options: 
	@echo "You need to invoke this Makefile like this:"
	@echo " 	make prod CORE=N - Build kernel module for target core 'N'"

prod:
ifndef CORE
	$(error Must define target core, ie. 'make prod CORE=15')
endif
	@echo "# Building ibstrace kernel module ... "
	$(MAKE) V=1 -C ibstrace/ prod CORE=$(CORE)
clean:
	$(MAKE) V=1 -C ibstrace/ clean
