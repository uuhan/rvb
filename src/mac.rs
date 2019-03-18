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
