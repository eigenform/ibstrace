SRC 	:= ibstrace.c
BIN 	:= ibstrace
CFLAGS 	:= -I$(CURDIR)/../include


all: $(BIN) tests

$(BIN): $(SRC)
	gcc $(CFLAGS) $(SRC) -o $@

tests: 
	as test.s -o test.elf
	objcopy -O binary -j .text test.elf test.bin

clean:
	rm -fv $(BIN)
