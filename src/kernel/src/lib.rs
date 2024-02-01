#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod heap;
mod idt;
mod textmode;
mod writer;

use crate::{idt::init_idt, textmode::VgaTextModeWriter};
use core::{arch::asm, hint::unreachable_unchecked, panic::PanicInfo};

extern crate alloc;

pub static mut WRITER: VgaTextModeWriter = VgaTextModeWriter::new();

#[no_mangle]
unsafe extern "C" fn kernel_main() -> ! {
    WRITER.row = 3;

    println!("Initializing IDT...");
    init_idt();

    println!("I'm about to page fault!");
    trigger_page_fault();
    println!("I survived a page fault!");

    // let n: u32 =  *( 0xea5dac6df70d2a7c as *const u32 );
    // println!("n = {}", n);

    // let x = Box::new(41);
    // println!("boxed int: {:?}", x);

    asm!("hlt");
    unreachable_unchecked();
}

fn trigger_page_fault() {
    unsafe{ *(0xdeadbeaf as *mut u64) = 1234 };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
