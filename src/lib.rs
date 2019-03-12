mod context;
mod ffi;
mod isolate;
mod platform;

pub use ffi::root::v8 as raw;

pub use ffi::root::v8::Local;
pub use ffi::root::v8::MaybeLocal;
pub use ffi::root::v8::Persistent;
pub use context::Context;
pub use isolate::Isolate;
pub use platform::Platform;

pub mod v8 {
    pub use crate::ffi::root::v8 as ffi;

    pub use crate::Local;
    pub use crate::MaybeLocal;
    pub use crate::Persistent;
    pub use crate::Context;
    pub use crate::Isolate;
    pub use crate::Platform;
}
