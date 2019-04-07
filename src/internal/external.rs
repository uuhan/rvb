use std::ffi::c_void;

use crate::v8::{
    raw,
    Local,
    Value,
    prelude::V8External,
};

pub use crate::v8::raw::External;

inherit!(External, Value);
inherit_local!(External, Value);

impl V8External {
    #[inline]
    pub fn New(data: *mut c_void) -> Self {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            External::New(isolate, data)
        }
    }

    #[inline]
    pub fn value(&self) -> *mut c_void {
        unsafe {
            self.Value()
        }
    }
}
