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
	cargo rustc --crate-type staticlib --release -- && \
	cp target/x86_64-atos/release/libatos_kernel.a ../../target/kernel/kernel.a && \
	cd ../..

build: $(asm_objects) kernel
	mkdir -p target && \
	ld -n -o target/atos-kernel.bin -T linker.ld $(asm_objects) target/kernel/kernel.a && \
	cp -v target/atos-kernel.bin iso/boot/atos-kernel.bin && \
	grub-mkrescue /usr/lib/grub/i386-pc -o target/atos.iso iso

clean:
	cd src/kernel && cargo clean
	rm -rf target/*

run:
	qemu-system-x86_64 -cdrom target/atos.iso -enable-kvm

dbg:
	qemu-system-x86_64 -no-reboot -no-shutdown -enable-kvm -d int -cdrom target/atos.iso -s -S & \
	gdb -ex "file target/atos-kernel.bin" -ex "target remote localhost:1234"

dev: build run;