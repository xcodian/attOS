/// exception info
/// pushed by the CPU before calling the exception handler
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExInfo {
    pub instruction_ptr: u64,
    pub code_seg: u64,
    pub rflags: u64,
    pub stack_ptr: u64,
    pub stack_seg: u64,
}

pub type ExHandlerFn =
    unsafe extern "C" fn(&ExInfo);

pub type ExHandlerWithCodeFn =
    unsafe extern "C" fn(&ExInfo, u64);

#[macro_export]
macro_rules! exc_handler {
    ($fn: ident) => {{
        use crate::interrupts::exceptions::ExHandlerFn;
        // check that the function is a valid one
        ($fn as ExHandlerFn);

        #[naked]
        unsafe extern "C" fn call_handler() {
            asm!(
                "mov rdi, rsp // move the stack pointer into first argument
                sub rsp, 8    // align new stack pointer
                call {}       // jump into the handler function
                add rsp, 8    // undo stack alignment
                iretq         // return from interrupt",
                sym $fn,
                options(noreturn)
            );
        }
        // return 64-bit pointer to handler fn
        call_handler as *const fn() as u64
    }};

    // +code = handler with exit code
    (+code $fn: ident) => {{
        use crate::interrupts::exceptions::ExHandlerWithCodeFn;
        // check that the function is a valid one
        ($fn as ExHandlerWithCodeFn);

        #[naked]
        unsafe extern "C" fn call_handler() {
            asm!(
                "pop rsi     // move the error code into the second argument
                mov rdi, rsp // move the stack pointer into first argument
                sub rsp, 8   // align new stack pointer
                call {}      // jump into the handler function
                add rsp, 8    // undo stack alignment
                iretq        // return from interrupt",
                sym $fn,
                options(noreturn)
            );
        }
        // return 64-bit pointer to handler fn
        call_handler as *const fn() as u64
    }};
}
