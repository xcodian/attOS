use core::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T: ?Sized> {
    is_locked: AtomicBool,
    data: T,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex {
            is_locked: AtomicBool::new(false),
            data,
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&mut self) -> MutexGuard<'_, T> {
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

pub struct MutexGuard<'m, T: ?Sized> {
    mutex: &'m mut Mutex<T>,
}

impl<'m, T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.mutex.data
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mutex.data
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
