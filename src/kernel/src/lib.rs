#![no_std]
#![no_main]

mod heap;
mod textmode;
mod writer;

use alloc::string::ToString;

use crate::{textmode::VgaTextModeWriter, writer::Writer};
use core::{arch::asm, hint::unreachable_unchecked, panic::PanicInfo};

extern crate alloc;

#[no_mangle]
unsafe extern "C" fn kernel_main() -> ! {
    let mut w = VgaTextModeWriter::new();
    // w.clear();
    w.row = 3;

    for i in 0..10 {
        for _ in 0..i {
            w.print(".");
        }
        w.println(" hoorayy!!");
    }

    let i = 1234i32;
    w.print("an int (will crash): ");
    let s = i.to_string();
    w.println(&s);

    asm!("hlt");
    unreachable_unchecked();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
