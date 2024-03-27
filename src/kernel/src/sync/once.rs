use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct Once<T> {
    data: UnsafeCell<Option<T>>,
    initialized: AtomicBool,
}

impl<T> Once<T> {
    pub const fn new() -> Self {
        Once {
            data: UnsafeCell::new(None),
            initialized: AtomicBool::new(false),
        }
    }

    pub fn get_or_init(&self, init_fn: impl FnOnce() -> T) -> &T
    {
        // swap false for true
        let exchange =
            self.initialized
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);

        match exchange {
            Ok(_) => {
                let mut_data = unsafe { &mut *self.data.get() };
                mut_data.get_or_insert_with(init_fn)
            }
            Err(is_initialized) => {
                // data must be initialized to be accessed
                assert!(is_initialized);

                // SAFETY: the data is proven to be initialized - it cannot
                // be modified anymore
                let option = unsafe { self.data.get().as_ref().unwrap() };
                option.as_ref().unwrap()
            }
        }
    }

    pub fn get(&self) -> Option<&T> {
        let is_initialized = self.initialized.load(Ordering::SeqCst);

        if is_initialized {
            let option = unsafe { self.data.get().as_ref().unwrap() };
            option.as_ref()
        } else {
            None
        }
    }
}

unsafe impl<T: Sync> Sync for Once<T> {}