use std::mem;
use crate::v8::{
    raw,
    raw::internal::{
        Address,
        kApiSystemPointerSize,
        Internals_kIsolateRootsOffset,
        Internals_kUndefinedValueRootIndex,
    },
    Local,
    Primitive,
};

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
