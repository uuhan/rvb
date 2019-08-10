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
        Internals_kEmptyStringRootIndex,
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

prim!(undefined, Internals_kUndefinedValueRootIndex);
prim!(null, Internals_kNullValueRootIndex);
prim!(r#true, Internals_kTrueValueRootIndex);
prim!(r#false, Internals_kFalseValueRootIndex);

// empty string
prim!(empty, Internals_kEmptyStringRootIndex, crate::v8::String);

/// V8_VERSION_STRING
pub fn version() -> &'static str {
    unsafe {
        CStr::from_ptr(V8_Version()).to_str().unwrap()
    }
}
