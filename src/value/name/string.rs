use std::ffi::CString;

use crate::v8::{
    raw,
    Local,
    Isolated,
    Name,
    Primitive,
    Value,
    Data,
    V8Value,
};

pub use raw::String;
pub type V8String = raw::String;

impl Local<String> {
    pub fn New<T: ToString>(string: T) -> Self {
        let cstr = CString::new(string.to_string());
        let isolate = Self::GetIsolate();
        unsafe {
            raw::String::NewFromUtf8(
                isolate,
                cstr.unwrap().as_ptr(),
                raw::String_NewStringType_kNormalString,
                -1
            )
        }
    }
}

inherit!(String, Name, Primitive, Value, Data);
inherit_local!(String, Name, Primitive, Value, Data);


impl V8Value for String {}
