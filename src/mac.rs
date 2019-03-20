macro_rules! deref {
    ($s:ident) => {
        impl Deref for $s {
            type Target = crate::v8::raw::$s;
            fn deref(&self) -> &Self::Target {
                unsafe { &*self.0 }
            }
        }

        impl DerefMut for $s {
            fn deref_mut(&mut self) -> &mut crate::v8::raw::$s {
                unsafe {
                    &mut *self.0
                }
            }
        }
    };
}

/// visit base class's methods
macro_rules! base {
    ($d: ident, $r:ty, $t:ty, $f:ident) => {
        fn $d(&mut self) -> $r {
            unsafe {
                let base = std::mem::transmute_copy::<&mut Self, &mut $t>(&self);
                base.$f()
            }
        }
    }
}
