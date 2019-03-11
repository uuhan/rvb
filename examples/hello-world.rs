use std::mem;
use std::ptr;
use v8_rs::v8;

pub fn main() {
    let platform = v8::Platform::new();
    let isolate = v8::Isolate::new();
    println!("{}", "v8");
}
