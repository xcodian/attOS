use core::ops::Deref;

use crate::sync::Once;

/// A lazily initialized value.
/// `&T` can be obtained by `*MY_LAZY`
pub struct Lazy<T> {
    data: Once<T>,
    init_fn: fn() -> T,
}

impl<T> Lazy<T> {
    pub const fn new(init_fn: fn() -> T) -> Self {
        Lazy {
            data: Once::new(),
            init_fn,
        }
    }

    pub fn get(&self) -> &T {
        self.data.get_or_init(self.init_fn)
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

#[macro_export]
macro_rules! atos_lazy_static {
    ($($v:vis static ref $N:ident: $T:ty = $e:expr;)*) => {
        $(
            $v static $N: crate::sync::Lazy<$T> = crate::sync::Lazy::new(|| $e);
        )*
    };
}
