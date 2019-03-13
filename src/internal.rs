use std::ops::Deref;
use std::ops::DerefMut;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;
use std::convert::Into;

pub use crate::v8::raw::Local;
pub use crate::v8::raw::MaybeLocal;
use crate::v8::raw;
use crate::v8::Value;
use crate::v8::Isolate;

extern "C" {
    pub fn V8_To_Local_Checked(value: raw::MaybeLocal<*mut c_void>) -> raw::Local<*mut c_void>;
}

pub trait PersistentValue<T> {}

pub trait Rooted {
    unsafe fn allocate() -> Self;
    unsafe fn enter(&mut self);
    unsafe fn exit(&mut self);
}

/// Object Should Live In an Isolate instance
pub trait Isolated {
    fn GetIsolate() -> Isolate {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            assert!(!isolate.is_null());
            Isolate(isolate)
        }
    }
}

impl<T> Local<T> {}

impl<T> Deref for Local<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.val_
        }
    }
}

impl<T> DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.val_
        }
    }
}

impl<T> Isolated for Local<T> {}

impl Into<String> for Local<Value> {
    fn into(self) -> String {
        let isolate = Local::<Value>::GetIsolate();
        unsafe {
            let ps = raw::String_Utf8Value::new(isolate.0, self).str_;
            CStr::from_ptr(ps).to_owned().into_string().unwrap_or(format!("{:?}", self))
        }
    }
}

impl<T> MaybeLocal<T> {
    pub fn to_local_checked(self) -> Local<T> {
        unsafe {
            mem::transmute(
                V8_To_Local_Checked(mem::transmute(self))
            )
        }
    }
}
