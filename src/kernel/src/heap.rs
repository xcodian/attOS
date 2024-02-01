use core::{alloc::GlobalAlloc, ptr::null_mut};

#[global_allocator]
static ALLOCATOR: NopAllocator = NopAllocator {};

pub struct NopAllocator;

unsafe impl GlobalAlloc for NopAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        panic!("dealloc should never be called")
    }
}
