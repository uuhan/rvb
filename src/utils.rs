use std::mem;
use std::ffi::CStr;
use std::os::raw::c_char;

use crate::v8::{
    raw,
    raw::internal::{
        Address,
        kApiSystemPointerSize,
        Internals_kIsolateRootsOffset,
        Internals_kUndefinedValueRootIndex,
        Internals_kNullValueRootIndex,
        Internals_kTrueValueRootIndex,
        Internals_kFalseValueRootIndex,
    },
    Local,
    Primitive,
};

extern {
    fn V8_Version() -> *const c_char;
}

/// Internals::GetRoot()
#[inline]
pub fn get_root(isolate: *const raw::Isolate, index: i32) -> *mut Address {
    unsafe {
        let isolate_address: Address = mem::transmute(isolate);
        let addr: Address = isolate_address
            + Internals_kIsolateRootsOffset as Address
            + (index as Address) * kApiSystemPointerSize as Address;

        mem::transmute(addr)
    }
}

/// v8::Undefined(Isolate* isolate)
pub fn undefined() -> Local<Primitive> {
    unsafe {
        let isolate: *const raw::Isolate = raw::Isolate_GetCurrent();
        let slot = get_root(isolate, Internals_kUndefinedValueRootIndex);
        mem::transmute(slot)
    }
}

/// v8::Null(Isolate* isolate)
pub fn null() -> Local<Primitive> {
    unsafe {
        let isolate: *const raw::Isolate = raw::Isolate_GetCurrent();
        let slot = get_root(isolate, Internals_kNullValueRootIndex);
        mem::transmute(slot)
    }
}

/// v8::True(Isolate* isolate)
pub fn r#true() -> Local<Primitive> {
    unsafe {
        let isolate: *const raw::Isolate = raw::Isolate_GetCurrent();
        let slot = get_root(isolate, Internals_kTrueValueRootIndex);
        mem::transmute(slot)
    }
}

/// v8::False(Isolate* isolate)
pub fn r#false() -> Local<Primitive> {
    unsafe {
        let isolate: *const raw::Isolate = raw::Isolate_GetCurrent();
        let slot = get_root(isolate, Internals_kFalseValueRootIndex);
        mem::transmute(slot)
    }
}

/// V8_VERSION_STRING
pub fn version() -> &'static str {
    unsafe {
        CStr::from_ptr(V8_Version()).to_str().unwrap()
    }
}
