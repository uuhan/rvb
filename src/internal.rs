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
    pub fn V8_Local_Is_Empty(value: raw::Local<*mut c_void>) -> bool;
}

pub struct Address(*mut raw::internal::Address);
pub trait PersistentValue<T> {}

/// an object can be enter in or exit out
pub trait Rooted {
    fn allocate() -> Self;
    fn enter(&mut self);
    fn exit(&mut self);
}

/// Object Should Live In an Isolate instance
pub trait Isolated {
    fn New() -> Box<Self> {
        unsafe {
            // TODO: seems not good
            mem::uninitialized()
        }
    }
    fn GetIsolate() -> Isolate {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            assert!(!isolate.is_null());
            Isolate(isolate)
        }
    }
}

/// isomorphism to v8::Local<T>
impl<T> Local<T> {
    pub fn is_empty(self) -> bool {
        unsafe {
            V8_Local_Is_Empty(mem::transmute(self))
        }
    }
}

/// local value auto deref
impl<T> Deref for Local<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.val_
        }
    }
}

/// local value auto deref
impl<T> DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.val_
        }
    }
}

/// local value lives in an isolate instance
impl<T> Isolated for Local<T> {}

/// cast local value into string
/// use v8::String::Utf8Value
impl Into<String> for Local<Value> {
    fn into(self) -> String {
        let isolate = Local::<Value>::GetIsolate();
        unsafe {
            let ps = raw::String_Utf8Value::new(isolate.0, self).str_;
            CStr::from_ptr(ps).to_owned().into_string().unwrap_or(format!("{:?}", self))
        }
    }
}

/// maybe local to local value
impl<T> MaybeLocal<T> {
    pub fn to_local_checked(self) -> Local<T> {
        unsafe {
            mem::transmute(
                V8_To_Local_Checked(mem::transmute(self))
            )
        }
    }
}
