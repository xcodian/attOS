use core::arch::asm;

pub mod idt;
pub mod exceptions;
pub mod handlers;
pub mod pic8259;
pub mod hardware;

pub fn enable_interrupts() {
    unsafe {
        asm!("sti", options(preserves_flags, nostack));
    }
}

pub fn disable_interrupts() {
    unsafe {
        asm!("cli", options(preserves_flags, nostack));
    }
}

pub fn without_interrupts<F: FnOnce() -> ()>(f: F) {
    disable_interrupts();
    f();
    enable_interrupts();
}