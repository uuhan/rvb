#![allow(non_snake_case)]
//! # v8_rs
//!
//! This crate aims to be the best bridge between Rust & V8 ;).
//! You can use this crate to power your app with js script support.
//!
//! NB: You should have V8 installed in your os and provide pkg-config config (v8.pc),
//! for example:
//!
//! ```sh
//! prefix=/home/v8/source/v8
//! Name: v8
//! Description: v8
//! Version: 7.5.0
//! Cflags: -I${prefix}/include
//! Libs: -L${prefix}/lib -lv8_monolith
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! # extern crate v8_rs;
//! use v8_rs::v8::{
//!     prelude::*,
//!     Platform,
//!     Isolate,
//! };
//!
//! fn main() {
//!     let _platform = Platform::New();
//!     let mut isolate = Isolate::New();
//!
//!     isolate.exec(|ctx| {
//!         // <do something>
//!         Ok(())
//!     }).unwrap();
//! }
//! ```
//!
//! See [examples](./examples) for more.
//!

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
        pub type V8Value = Local<crate::v8::Value>;
        pub type V8Name = Local<crate::v8::Name>;
        pub type V8External = Local<crate::v8::External>;
        pub type V8String = Local<crate::v8::String>;
        pub type V8Object = Local<crate::v8::Object>;
        pub type V8Function = Local<crate::v8::Function>;
        pub type V8Private = Local<crate::v8::Private>;
        pub type V8Signature = Local<crate::v8::Signature>;
        pub type V8Script = Local<crate::v8::Script>;
        pub type V8Context = Local<crate::v8::Context>;
    }
}
