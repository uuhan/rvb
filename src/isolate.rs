use crate::raw;

extern "C" {
    pub fn V8_Isolate_New() -> *mut raw::Isolate;
    pub fn V8_Isolate_Dispose(isolate: *mut raw::Isolate);
    pub fn V8_Isolate_Enter(isolate: *mut raw::Isolate);
    pub fn V8_Isolate_Exit(isolate: *mut raw::Isolate);

}
#[repr(C)]
pub struct Isolate {
    handler: *mut raw::Isolate,
}

impl Isolate {
    pub fn new() -> Self {
        let isolate = unsafe { V8_Isolate_New() };
        assert!(!isolate.is_null());
        Self { handler: isolate }
    }
}
