#![allow(non_snake_case)]
#[macro_use]
pub(crate) mod mac;

mod context;
mod ffi;
mod isolate;
mod platform;
mod runtime;
mod scope;
mod script;
mod internal;
mod value;
mod object;
mod function;

pub mod v8 {
    pub use crate::ffi::root::v8 as raw;
    pub use raw::Value;

    pub use crate::context::Context;
    pub use crate::context::ContextParams;
    pub use crate::isolate::Isolate;
    pub use crate::isolate::ISOLATE_DATA_SLOT;
    pub use crate::platform::Platform;
    pub use crate::scope::HandleScope;
    pub use crate::scope::ContextScope;
    pub use crate::script::Script;
    pub use crate::object::*;
    pub use crate::function::*;
    pub use crate::value::*;

    pub use crate::internal::*;

    pub mod prelude {
        pub use crate::internal::*;
    }
}
