# atOS: Another Tiny Operating System
atOS is a research operating system made for fun and educational purposes, like many of its ancestors.

### Install Dependencies

atOS needs the following build dependencies:
- `make`
- `ld` for x86_64
- `grub` + `xorriso`
- `qemu` with x86_64 support
- `cargo` + `rustc` and a `nightly` Rust toolchain

On Arch Linux, these dependencies can be installed with the command
```
$ sudo pacman -S base-devel grub xorriso rustup qemu-full
$ rustup install nightly
```

### Build & Run
You may build the bootable `.iso` file with
```
$ make
```

You may build and emulate atOS in one command with
```
$ make run
```

### Directory Layout
- `iso` - filesystem of the `.iso` file of the OS
- `src` - source code
  - `init` - multiboot2 entry point that performs setup and jumps into the kernel
  - `kernel` - kernel written in Rust
- `target` - compiled artifacts
  - `atos-kernel.bin` - linked kernel binary
  - `atos.iso` - bootable OS image

### References
- https://wiki.osdev.org/Multiboot
- https://wiki.osdev.org/Setting_Up_Long_Mode
- https://www.youtube.com/watch?v=FkrpUaGThTQ
- https://www.youtube.com/watch?v=wz9CZBeXR6U
- https://wiki.osdev.org/Porting_Rust_standard_library
- https://os.phil-opp.com/