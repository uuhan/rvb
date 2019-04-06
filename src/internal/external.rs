use std::ffi::c_void;

use crate::v8::{
    raw,
    Local,
    Value,
};

pub use crate::v8::raw::External;

inherit!(External, Value);
inherit_local!(External, Value);

impl Local<External> {
    pub fn New(data: *mut c_void) -> Self {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            External::New(isolate, data)
        }
    }
}
