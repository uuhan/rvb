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
pub mod utils;

pub mod v8 {
    pub use crate::utils;
    pub use crate::ffi::root::v8 as raw;

    pub use crate::context::Context;
    pub use crate::context::ContextParams;
    pub use crate::isolate::Isolate;
    pub use crate::isolate::IsolateData;
    pub use crate::isolate::Unlocker;
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
        pub type ObjectT = Local<crate::v8::ObjectTemplate>;
        pub type FunctionT = Local<crate::v8::FunctionTemplate>;
    }
}
