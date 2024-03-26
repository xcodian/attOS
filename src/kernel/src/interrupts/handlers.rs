use crate::{interrupts::pic8259::notify_end_of_interrupt, print};

use super::exceptions::ExInfo;
use crate::ctlregs;

pub unsafe extern "C" fn kpage_fault(info: &ExInfo, error_code: u64) {
    let bad_ptr: u64 = ctlregs::read_cr2();

    panic!(
        "page fault at {:#x} (ptr={:#x} st={:#x} err={} cs={:#x} rflags={:#x})",
        info.instruction_ptr, bad_ptr, info.stack_ptr, error_code, info.code_seg, info.rflags
    );
}

pub unsafe extern "C" fn kdouble_fault(info: &ExInfo, error_code: u64) {
    panic!(
        "double fault at {:#x} (st={:#x} err={} cs={:#x} rflags={:#x})",
        info.instruction_ptr, info.stack_ptr, error_code, info.code_seg, info.rflags
    );
}

pub unsafe extern "C" fn kgprot_fault(info: &ExInfo, error_code: u64) {
    panic!(
        "general protection fault at {:p} (st={:p} err={:#x} cs={:#x} rflags={:#x} ss={:p})",
        info.instruction_ptr as *const (),
        info.stack_ptr as *const (),
        error_code,
        info.code_seg,
        info.rflags,
        info.stack_seg as *const ()
    );
}

pub unsafe extern "C" fn kinvalid_opcode(info: &ExInfo) {
    panic!(
        "invalid opcode at {:p} (st={:p} cs={:#x} rflags={:#x} ss={:p})",
        info.instruction_ptr as *const (),
        info.stack_ptr as *const (),
        info.code_seg,
        info.rflags,
        info.stack_seg as *const ()
    );
}