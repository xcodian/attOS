use core::ops::Deref;

use super::pic8259::{notify_end_of_interrupt, PIC1_OFFSET};

pub mod timer;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HwInterrupt {
    Timer = PIC1_OFFSET,
    Keyboard
}

impl HwInterrupt {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }

    pub unsafe fn notify_end(self) {
        notify_end_of_interrupt(self.as_u8())
    }
}