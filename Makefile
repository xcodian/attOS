asm_sources := $(shell find src/ -name *.asm)
asm_objects := $(patsubst src/%.asm, target/%.o, $(asm_sources))

$(asm_objects): target/%.o : src/%.asm
	mkdir -p $(dir $@) && \
	nasm -f elf64 $(patsubst target/%.o, src/%.asm, $@) -o $@

.PHONY: build
build: $(asm_objects)
	mkdir -p target && \
	ld -n -o target/atos-kernel.bin -T linker.ld $(asm_objects) && \
	cp -v target/atos-kernel.bin iso/boot/atos-kernel.bin && \
	grub-mkrescue /usr/lib/grub/i386-pc -o target/atos.iso iso

clean:
	rm -rv target/*