use std::ffi::CString;
use cfg_if::cfg_if;

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
#[cfg(any(feature = "7_6_0", feature = "7_8_0"))]
pub use raw::NewStringType_kNormal as NewNormalStringType;

pub use raw::String;

impl Local<String> {
    pub fn New<T: ToString>(string: T) -> Self {
        let cstr = CString::new(string.to_string());
        let isolate = Self::GetIsolate();

        cfg_if! {
            if #[cfg(feature = "7_4_0")] {
                unsafe {
                    raw::String::NewFromUtf8(
                        isolate,
                        cstr.unwrap().as_ptr(),
                        NewNormalStringType as u32,
                        -1
                    )
                }
            } else if #[cfg(feature = "7_5_0")] {
                unsafe {
                    raw::String::NewFromUtf8(
                        isolate,
                        cstr.unwrap().as_ptr(),
                        NewNormalStringType,
                        -1
                    )
                }
            } else if #[cfg(any(feature = "7_6_0", feature = "7_8_0"))] {
                unsafe {
                    raw::String::NewFromUtf8(
                        isolate,
                        cstr.unwrap().as_ptr(),
                        NewNormalStringType,
                        -1
                    ).to_local_checked().unwrap()
                }
            } else {
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
    }
}

inherit!(String, Name, Primitive, Value, Data);
inherit_local!(String, Name, Primitive, Value, Data);

impl ValueTrait for String {}
