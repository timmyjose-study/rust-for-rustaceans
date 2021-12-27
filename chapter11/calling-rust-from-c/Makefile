CC := gcc
CFLAGS := -std=c11 -Wall -Werror -O2

EXE:=calc

all:
	$(CC) $(CFLAGS) main.c -o $(EXE) ../target/debug/libcalc.a

.PHONY: clean
clean:
	rm -rf $(EXE)