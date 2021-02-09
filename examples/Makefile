# Do not remove .wasm files
.PRECIOUS: %.wasm

CC = clang
WASM2WAT ?= wasm2wat

export PATH := /usr/local/opt/llvm/bin:$(PATH)

%.wasm: %.c
	"$(CC)" -nostdlib --target=wasm32 -Wl,--allow-undefined -o "$(basename $<).wasm" $(CFLAGS) "$<"

%.wat: %.wasm
	"$(WASM2WAT)" $(WASM2WATFLAGS) "$<" > "$(basename $<).wat"

.PHONY: all
all:
	make $(patsubst %.c,%.wat,$(wildcard *.c))

.PHONY: clean
clean:
	rm -f *.wat *.wasm
