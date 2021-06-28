SRC 	:= ibstrace.c
BIN 	:= ibstrace
CFLAGS 	:= -I$(CURDIR)/../include


all: $(BIN)

$(BIN): $(SRC)
	gcc $(CFLAGS) $(SRC) -o $@

clean:
	rm -fv $(BIN)
