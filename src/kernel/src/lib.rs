#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod heap;
mod textmode;
mod writer;

use alloc::{boxed::Box, string::ToString};

use crate::{textmode::VgaTextModeWriter, writer::Writer};
use core::{arch::asm, hint::unreachable_unchecked, panic::PanicInfo};

extern crate alloc;

pub static mut WRITER: VgaTextModeWriter = VgaTextModeWriter::new();

#[no_mangle]
unsafe extern "C" fn kernel_main() -> ! {
    WRITER.row = 3;

    let x = Box::new(41);
    println!("boxed int: {:?}", x);

    asm!("hlt");
    unreachable_unchecked();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
