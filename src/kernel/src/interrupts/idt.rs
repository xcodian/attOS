use core::{
    arch::asm, mem::size_of, ops::{Deref, DerefMut}, pin::Pin, ptr::addr_of
};

use crate::{
    atos_lazy_static, exc_handler,
    interrupts::{
        handlers::{kdouble_fault, kgprot_fault, kinvalid_opcode, kpage_fault},
        hardware::{keyboard::keyboard_interrupt, timer::timer_interrupt, HwInterrupt},
    },
    println,
    sync::Mutex,
};

use super::{enable_interrupts, pic8259::init_chained_pic8259};

atos_lazy_static!(
    pub static ref IDT: Mutex<Idt> = Mutex::new(Idt::new());
    pub static ref IDT_REF: IdtRef = {
        let idt_addr = &*IDT.lock() as *const Idt as u64;

        IdtRef {
            ptr: idt_addr,
            limit: (size_of::<Idt>() - 1) as u16,
        }
    };
);

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    pub isr_low: u16,
    pub kernel_cs: u16,
    pub ist: u8,
    pub attributes: u8,
    pub isr_mid: u16,
    pub isr_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn new(addr: u64, flags: u8) -> Self {
        Self {
            isr_low: addr as u16,
            kernel_cs: 0x08, // todo: load CS here instead of hardcoding
            ist: 0,
            attributes: flags,
            isr_mid: (addr >> 16) as u16,
            isr_high: (addr >> 32) as u32,
            reserved: 0,
        }
    }

    pub const fn null() -> Self {
        Self::new(0, 0x8E)
    }
}

#[repr(C, packed(2))]
pub struct IdtRef {
    limit: u16,
    ptr: u64,
}

pub type IdtEntries = [IdtEntry; 256];

#[repr(C, packed)]
pub struct Idt(IdtEntries);

impl Idt {
    const fn new() -> Self {
        Self([IdtEntry::null(); 256])
    }
}

impl Deref for Idt {
    type Target = IdtEntries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Idt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

extern "C" fn isr_stub() {
    println!("isr_stub: unhandled interrupt!");
}

pub unsafe fn init_idt() {
    {
        let mut idt = IDT.lock();
    
        for i in 0..256 {
            idt[i] = IdtEntry::new(isr_stub as *const () as u64, 0x8E);
        }
    
        // register interrupt handlers
        // https://wiki.osdev.org/Exceptions
        idt[0x6] = IdtEntry::new(exc_handler!(kinvalid_opcode), 0x8E);
        idt[0x8] = IdtEntry::new(exc_handler!(+code kdouble_fault), 0x8E);
        idt[0xD] = IdtEntry::new(exc_handler!(+code kgprot_fault), 0x8E);
        idt[0xE] = IdtEntry::new(exc_handler!(+code kpage_fault), 0x8E);
    
        idt[HwInterrupt::Timer.as_usize()] = IdtEntry::new(exc_handler!(timer_interrupt), 0x8E);
        idt[HwInterrupt::Keyboard.as_usize()] = IdtEntry::new(exc_handler!(keyboard_interrupt), 0x8E);
    }

    // load the table
    let idt_ref = &*IDT_REF as *const IdtRef;
    asm!("lidt [{}]", in(reg) idt_ref, options(readonly, nostack, preserves_flags));
}

pub fn setup_interrupts() {
    unsafe {
        init_idt();
        init_chained_pic8259();
        enable_interrupts();
    }
}
