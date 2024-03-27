use core::{
    cell::UnsafeCell, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, Ordering}
};

#[derive(Debug)]
pub struct Mutex<T: ?Sized> {
    is_locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex {
            is_locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        // spin
        while !self.try_lock() {}

        return MutexGuard {
            mutex: self,
        }
    }

    fn try_lock(&self) -> bool {
        // try swap "false" for "true"
        let result = self.is_locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
        return result.is_ok();
    }

    fn unlock(&self) {
        self.is_locked.store(false, Ordering::Release);
    }
}

// if T is Send, Mutex<T> is Send + Sync
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

#[derive(Debug)]
pub struct MutexGuard<'m, T: ?Sized> {
    mutex: &'m Mutex<T>,
}

impl<'m, T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: only one guard can be active at a time
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: only one guard can be active at a time
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

// if T is Sync, MutexGuard<T> is Sync
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}