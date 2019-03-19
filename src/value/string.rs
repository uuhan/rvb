use std::ffi::CString;
use std::mem;
use std::convert::Into;

use crate::v8::{
    raw,
    Local,
    Isolated,
    Name,
    Data,
};

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

impl Into<Local<Name>> for Local<String> {
    fn into(self) -> Local<Name> {
        unsafe {
            mem::transmute::<Local<String>, Local<Name>>(self)
        }
    }
}

impl Into<Local<Data>> for Local<String> {
    fn into(self) -> Local<Data> {
        unsafe {
            mem::transmute::<Local<String>, Local<Data>>(self)
        }
    }
}
