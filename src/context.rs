use crate::raw;

extern "C" {
    pub fn V8_Context_New(output: *mut raw::Context);
    pub fn V8_Context_Enter(input: *mut raw::Context);
    pub fn V8_Context_Exit(input: *mut raw::Context);
}
#[repr(C)]
pub struct Context(*mut raw::Context);
