use std::mem;
use std::ptr;
use v8_ffi::V8;

pub fn main() {
    let platform = V8::Platform::new();
    let isolate = V8::Isolate::new();
    println!("{}", "v8");
}
