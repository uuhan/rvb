macro_rules! deref_mut {
    ($s:ident) => {
        impl ::std::ops::Deref for $s {
            type Target = crate::v8::raw::$s;
            fn deref(&self) -> &Self::Target {
                unsafe { &*self.0 }
            }
        }

        impl ::std::ops::DerefMut for $s {
            fn deref_mut(&mut self) -> &mut crate::v8::raw::$s {
                unsafe {
                    &mut *self.0
                }
            }
        }
    };
}

macro_rules! deref {
    ($s:ident) => {
        impl ::std::ops::Deref for $s {
            type Target = crate::v8::raw::$s;
            fn deref(&self) -> &Self::Target {
                unsafe { &*self.0 }
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

/// down cast to base class
/// NB: c++ class inherit only
macro_rules! inherit {
    ($source:ty, $base:ty) => {
        impl ::std::convert::Into<$base> for $source {
            fn into(self) -> $base {
                unsafe {
                    ::std::mem::transmute::<Self, $base>(self)
                }
            }
        }
    }
}

/// down cast to base class(inside Local<T>)
macro_rules! inherit_local {
    ($source:ty, $base:ty) => {
        impl ::std::convert::Into<crate::v8::Local<$base>> for crate::v8::Local<$source> {
            fn into(self) -> crate::v8::Local<$base> {
                unsafe {
                    ::std::mem::transmute::<Self, crate::v8::Local<$base>>(self)
                }
            }
        }
    }
}
