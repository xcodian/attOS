#![no_std]
#![no_main]

use core::{arch::asm, hint::unreachable_unchecked, panic::PanicInfo};

static mut VIDEO_MEM: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
unsafe extern "C" fn kernel_main() -> ! {
    let line_4 = VIDEO_MEM.offset(160 * 4);
    let string = b"Hello from Rust!";

    for (i, &byte) in string.iter().enumerate() {
        *line_4.offset(i as isize * 2) = byte;
        *line_4.offset(i as isize * 2 + 1) = 0xa;
    }

    asm!("hlt");
    unreachable_unchecked();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}