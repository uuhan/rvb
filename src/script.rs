use std::ptr;
use crate::v8::{Local, Context, raw::Value, String, raw};

pub use raw::Script;

impl Local<Script> {
    pub fn New(context: Local<Context>, source: Local<String>) -> Self {
        unsafe {
            raw::Script::Compile(context, source, ptr::null_mut()).to_local_checked()
        }
    }

    pub fn run (&self, context: Local<Context>) -> Local<Value> {
        unsafe {
            (*self.val_).Run(context).to_local_checked()
        }
    }
}
