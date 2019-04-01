#![allow(dead_code, unused_variables)]
// internal utils
use crate::v8::raw;

pub struct Internals;

impl Internals {
    pub fn GetRoot(isolate: *const raw::Isolate, index: u32) -> *const raw::internal::Address {
        unimplemented!()
    }
}
