asm_sources := $(shell find src/ -name *.asm)
asm_objects := $(patsubst src/%.asm, target/%.o, $(asm_sources))

.PHONY: build default
default: build ;

$(asm_objects): target/%.o : src/%.asm
	mkdir -p $(dir $@) && \
	nasm -f elf64 $(patsubst target/%.o, src/%.asm, $@) -o $@

kernel:
	mkdir -p target/kernel && \
	rm -f target/kernel/* && \
	cd src/kernel && \
	cargo rustc --release -- --emit=obj && \
	find target/x86_64-atos/release/deps -name '*.o' -exec cp {} ../../target/kernel \; && \
	cd ../..

build: $(asm_objects) kernel
	mkdir -p target && \
	ld -n -o target/atos-kernel.bin -T linker.ld $(asm_objects) $(shell find target/kernel -name '*.o') && \
	cp -v target/atos-kernel.bin iso/boot/atos-kernel.bin && \
	grub-mkrescue /usr/lib/grub/i386-pc -o target/atos.iso iso

clean:
	cd src/kernel && cargo clean
	rm -rf target/*

run: build
	qemu-system-x86_64 -cdrom target/atos.iso