#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![feature(asm_const)]

mod ctlregs;
mod heap;
mod interrupts;
mod panic;
mod port;
mod textmode;
mod writer;

use interrupts::idt::setup_interrupts;

use crate::textmode::VgaTextModeWriter;

extern crate alloc;

pub static mut WRITER: VgaTextModeWriter = VgaTextModeWriter::new();

#[no_mangle]
unsafe extern "C" fn kernel_main() -> ! {
    WRITER.row = 3;

    setup_interrupts();

    println!("[ ok ] set up interrupts");
    neofetch();

    println!("The PIC timer interrupt is creating these dots:");

    loop {}
}

fn neofetch() {
    let art = r#"
                 .d88b,  MMMMMMMMMMM MMMMMMMMMMM
           qq         8  MMP"""""YMM MP""""""`MM 
           88    d8888P  M' .mmm. `M M  mmmmm..M 
.d8888b. d8888P  8       M  MMMMM  M M.      `YM 
88'  `88   88    `8888'  M  MMMMM  M MMMMMMM.  M 
88.  .88   88            M. `MMM' .M M. .MMM'  M 
`88888P8   dP            MMb.   .dMM Mb.     .dM 
                         MMMMMMMMMMM MMMMMMMMMMM 

      A Tiny Test Operating System
        v0.1 by Martin Velikov
    "#;

    println!("{}", art);
}
