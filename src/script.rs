use std::ptr;
use crate::v8::{MaybeLocal, Local, Context, raw::Value, String, raw};

pub use raw::Script;

impl Local<Script> {
    pub fn New(context: Local<Context>, source: Local<String>) -> Self {
        unsafe {
            raw::Script::Compile(context, source, ptr::null_mut()).to_local_checked()
        }
    }

    pub fn run (&mut self, context: Local<Context>) -> MaybeLocal<Value> {
        unsafe {
            self.Run(context)
        }
    }
}
