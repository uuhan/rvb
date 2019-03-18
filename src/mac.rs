macro_rules! deref {
    ($s:ident) => {
        impl Deref for $s {
            type Target = raw::$s;
            fn deref(&self) -> &Self::Target {
                unsafe { &*self.0 }
            }
        }

        impl DerefMut for $s {
            fn deref_mut(&mut self) -> &mut raw::$s {
                unsafe {
                    &mut *self.0
                }
            }
        }
    };

    ($s:ident -> $handle:ident) => {
        impl Deref for $s {
            type Target = raw::$s;
            fn deref(&self) -> &Self::Target {
                unsafe { &*self.$handle }
            }
        }

        impl DerefMut for $s {
            fn deref_mut(&mut self) -> &mut raw::$s {
                unsafe {
                    &mut *self.$handle
                }
            }
        }
    };
}
