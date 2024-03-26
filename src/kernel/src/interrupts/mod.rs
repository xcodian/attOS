use core::arch::asm;

pub mod idt;
pub mod exceptions;
pub mod handlers;
pub mod pic8259;
pub mod hardware;

pub unsafe fn enable_interrupts() {
    asm!("sti", options(preserves_flags, nostack));
}

pub unsafe fn disable_interrupts() {
    asm!("cli", options(preserves_flags, nostack));
}