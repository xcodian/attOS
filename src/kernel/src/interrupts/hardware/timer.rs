use crate::{
    interrupts::{exceptions::ExInfo, hardware::HwInterrupt},
    print,
};

pub unsafe extern "C" fn timer_interrupt(_: &ExInfo) {
    print!(".");
    HwInterrupt::Timer.notify_end();
}
