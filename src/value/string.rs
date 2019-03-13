use std::ffi::CString;

use crate::v8::{raw, Local, Isolated};

pub use raw::String;

impl Local<String> {
    pub fn New<T: ToString>(string: T) -> Self {
        let cstr = CString::new(string.to_string());
        let isolate = Self::GetIsolate();
        unsafe {
            raw::String::NewFromUtf8(
                isolate.0,
                cstr.unwrap().as_ptr(),
                raw::String_NewStringType_kNormalString,
                -1
            )
        }
    }
}

