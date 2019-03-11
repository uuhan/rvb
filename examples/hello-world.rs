use std::mem::uninitialized;
use v8_ffi::v8;

pub fn main() {
    unsafe {
        v8::V8_Initialize_Platform();
        v8::V8_Initialize();

        v8::V8_Dispose();
        v8::V8_Free_Platform();
    }
    println!("{}", "v8");
}
