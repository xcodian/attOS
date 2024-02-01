use core::{
    arch::asm,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::addr_of,
};

use crate::println;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    isr_low: u16,
    kernel_cs: u16,
    ist: u8,
    attributes: u8,
    isr_mid: u16,
    isr_high: u32,
    _reserved: u32,
}

impl IdtEntry {
    pub const fn new(addr: u64, flags: u8) -> Self {
        Self {
            isr_low: (addr & 0xFFFF) as u16,
            kernel_cs: 0, // todo: figure this out
            ist: 0,
            attributes: flags,
            isr_mid: ((addr >> 16) & 0xFFFF) as u16,
            isr_high: ((addr >> 32) & 0xFFFFFFFF) as u32,
            _reserved: 0,
        }
    }

    pub const fn null() -> Self {
        Self::new(0, 0x8E)
    }
}

#[repr(C, packed(2))]
pub struct IdtRef {
    limit: u16,
    base: *const Idt,
}

pub type IdtEntries = [IdtEntry; 256];

#[repr(transparent)]
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

static mut IDT: Idt = Idt::new();
static mut IDT_REF: IdtRef = IdtRef {
    base: unsafe { addr_of!(IDT) },
    limit: (size_of::<Idt>() - 1) as u16,
};

pub fn init_idt() {
    for i in 0..32 {
        let fn_addr = isr_stub as *const fn() as u64;
        let entry = IdtEntry::new(fn_addr, 0x8E);
        unsafe {
            IDT[i] = entry;
        }
    }

    unsafe {
        asm!("lidt [{}]", in(reg) addr_of!(IDT_REF), options(readonly, nostack, preserves_flags));
    }
}

extern "C" fn isr_stub() -> ! {
    println!("hit isr_stub()!");
    loop {}
}
