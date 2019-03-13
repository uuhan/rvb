#[macro_use]
pub(crate) mod mac;

mod context;
mod ffi;
mod isolate;
mod platform;
mod runtime;
mod scope;

pub use ffi::root::v8 as raw;

pub use context::Context;
pub use isolate::Isolate;
pub use platform::Platform;
pub use scope::HandleScope;

pub mod v8 {
    pub use crate::ffi::root::v8 as raw;

    pub use crate::Context;
    pub use crate::Isolate;
    pub use crate::Platform;
    pub use crate::HandleScope;

    pub use crate::Local;
    pub use crate::MaybeLocal;
    pub use crate::Persistent;
    pub use crate::Root;
}

pub trait Local<T> {}

pub trait MaybeLocal<T> {
    fn to_local_checked() -> dyn Local<T>;
}

pub trait Persistent<T> {}

pub trait Root {
    unsafe fn allocate() -> Self;
    unsafe fn enter(&mut self);
    unsafe fn exit(&mut self);
}

/// Object Should Live In an Isolate instance
pub trait Isolated {
    fn new() -> Self;

    fn isolate() -> Isolate {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            assert!(!isolate.is_null());
            Isolate(isolate)
        }
    }
}
