#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![allow(dead_code)]

mod ctlregs;
mod heap;
mod interrupts;
mod panic;
mod port;
mod sync;
mod textmode;
mod writer;

use core::arch::asm;
use core::hint::unreachable_unchecked;

use crate::interrupts::idt::setup_interrupts;
use crate::interrupts::without_interrupts;
use crate::sync::Mutex;
use crate::textmode::VgaTextModeWriter;

extern crate alloc;

atos_lazy_static! {
    pub static ref MY_LAZY: i32 = 1;
    pub static ref WRITER: Mutex<VgaTextModeWriter> = Mutex::new(VgaTextModeWriter::new());
}

#[no_mangle]
unsafe extern "C" fn kernel_main() -> ! {
    // WRITER.lock().row = 3;

    setup_interrupts();

    println!("[ ok ] set up interrupts");
    neofetch();

    println!("The PIC timer interrupt is running:");
    
    loop {
        asm!("hlt");
    }
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

      A Tiny (Test) Operating System
        v0.1 by Martin Velikov
    "#;

    println!("{}", art);
}
