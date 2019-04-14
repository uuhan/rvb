#![allow(unused_imports)]
mod utils;
mod external;
mod persistent;
mod value;
mod error;

use std::ops::Deref;
use std::ops::DerefMut;
use std::marker::PhantomData;
use std::ptr;
use std::default::Default;
use std::ffi::c_void;
use std::mem;
use std::convert::Into;

pub(crate) use utils::*;
pub use external::*;
pub use persistent::*;
pub use value::*;
pub use error::*;

use crate::v8::raw;
pub use crate::v8::raw::{
    Local,
    MaybeLocal,
    Maybe,
    DeserializeInternalFieldsCallback,
    TryCatch,
    Name,
    Data,
    Template,

    PropertyAttribute,
    PropertyAttribute_None,
    PropertyAttribute_ReadOnly,
    PropertyAttribute_DontEnum,
    PropertyAttribute_DontDelete,
};

pub struct Address(*mut raw::internal::Address);
pub trait PersistentValue<T> {}

/// isomorphism to v8::Template base class
pub trait TemplateTrait {
    /// NB: use std::mem::transmute_copy to reinterpert_cast class to it's base class
    /// should only impl this trait for ObjectTemplate & FunctionTemplate...
    fn set<T1: Into<Local<Name>>, T2: Into<Local<Data>>>
        (&mut self, name: T1, value: T2) {
            unsafe {
                let base = mem::transmute_copy::<&mut Self, &mut Template>(&self);
                base.Set(
                    name.into(),
                    value.into(),
                    PropertyAttribute_None)
            }
        }
}

/// an object can be enter in or exit out
pub trait Rooted {
    #[inline]
    fn allocate() -> Self;
    #[inline]
    fn enter(&mut self);
    #[inline]
    fn exit(&mut self);
}

/// Object Should Live In an Isolate instance
pub trait Isolated<'a> {
    #[inline]
    fn New() -> Box<Self> {
        unsafe {
            // TODO: seems not good
            mem::uninitialized()
        }
    }

    #[inline]
    fn GetIsolate() -> &'a mut raw::Isolate {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            isolate.as_mut().unwrap()
        }
    }
}

/// isomorphism to v8::Local<T>
impl<T> Local<T> {
    /// empty Local<T>
    /// v8::Local<T>()
    pub fn Empty() -> Self {
        Local {
            val_: ptr::null_mut(),
            _phantom_0: PhantomData,
        }
    }

    /// if Local<T> is empty
    pub fn is_empty(self) -> bool {
        self.val_.is_null()
    }
}

pub trait LocalValue {
    type Item;
}

/// Local Value Trait
impl<T> LocalValue for Local<T> {
    type Item = T;
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
impl<'a, T> Isolated<'a> for Local<T> {}

/// TryCatch Isolated
impl<'a> Isolated<'a> for TryCatch {
    fn New() -> Box<Self> {
        let isolate = Self::GetIsolate();
        Box::new(
            unsafe {
                TryCatch::new(isolate)
            }
        )
    }
}

/// maybe local to local value
impl<T> MaybeLocal<T> {
    /// empty MaybeLocal<T>
    /// v8::MaybeLocal<T>()
    pub fn Empty() -> Self {
        MaybeLocal {
            val_: ptr::null_mut(),
            _phantom_0: PhantomData,
        }
    }

    /// if this maybelocal value is nothing
    pub fn is_empty(&self) -> bool {
        self.val_.is_null()
    }

    /// unwrap maybelocal to local
    pub fn to_local_checked(&self) -> V8Result<Local<T>> {
        if self.is_empty() {
            Err(V8Error::V8EmptyMaybeLocalErr)
        } else {
            Ok(Local {
                val_: self.val_,
                _phantom_0: PhantomData,
            })
        }
    }
}

/// convert Local<T> to MaybeLocal<T>
impl<T> Into<MaybeLocal<T>> for Local<T> {
    fn into(self) -> MaybeLocal<T> {
        let Local {
            val_, _phantom_0,
        } = self;

        MaybeLocal {
            val_,
            _phantom_0,
        }
    }
}

/// Default DeserializeInternalFieldsCallback
impl Default for DeserializeInternalFieldsCallback {
    fn default() -> Self {
        DeserializeInternalFieldsCallback {
            callback: None,
            data: ptr::null_mut(),
        }
    }
}
