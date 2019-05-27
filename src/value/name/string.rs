use std::ffi::CString;

use crate::v8::{
    raw,
    Local,
    Isolated,
    Name,
    Primitive,
    Value,
    Data,
    ValueTrait,
};

#[cfg(feature = "7_5_0")]
pub use raw::String_NewStringType_kNormalString as NewNormalStringType;
#[cfg(feature = "7_6_0")]
pub use raw::NewStringType_kNormal as NewNormalStringType;

pub use raw::String;

impl Local<String> {
    pub fn New<T: ToString>(string: T) -> Self {
        let cstr = CString::new(string.to_string());
        let isolate = Self::GetIsolate();
        #[cfg(feature = "7_5_0")]
        unsafe {
            raw::String::NewFromUtf8(
                isolate,
                cstr.unwrap().as_ptr(),
                NewNormalStringType,
                -1
            )
        }
        #[cfg(feature = "7_6_0")]
        unsafe {
            raw::String::NewFromUtf8(
                isolate,
                cstr.unwrap().as_ptr(),
                NewNormalStringType,
                -1
            ).to_local_checked().unwrap()
        }
    }
}

inherit!(String, Name, Primitive, Value, Data);
inherit_local!(String, Name, Primitive, Value, Data);

impl ValueTrait for String {}
