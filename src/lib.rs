mod ffi;

pub mod v8 {
    pub use crate::ffi::root::v8::*;

    extern "C" {
        pub fn V8_Initialize_Platform() -> bool;
        pub fn V8_Initialize() -> bool;
        pub fn V8_Dispose() -> bool;
        pub fn V8_Free_Platform() -> bool;

        pub fn V8_Isolate_New() -> bool;
        pub fn V8_Isolate_Dispose();
        pub fn V8_Isolate_Enter();
        pub fn V8_Isolate_Exit();

        pub fn V8_Context_New();
        pub fn V8_Context_Enter();
        pub fn V8_Context_Exit();
    }
}
