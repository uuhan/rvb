use crate::raw;
use std::mem;

extern "C" {
    pub fn V8_Isolate_Dispose(isolate: *mut raw::Isolate);
    pub fn V8_Isolate_Enter(isolate: *mut raw::Isolate);
    pub fn V8_Isolate_Exit(isolate: *mut raw::Isolate);

}
#[repr(C)]
pub struct Isolate(pub *mut raw::Isolate);

impl Isolate {
    pub fn new() -> Self {
        let isolate = unsafe {
            let mut create_params: raw::Isolate_CreateParams = mem::zeroed();
            create_params.array_buffer_allocator = raw::ArrayBuffer_Allocator::NewDefaultAllocator();
            raw::Isolate::New(&create_params)
        };
        assert_eq!(false, isolate.is_null());
        Self(isolate)
    }

    pub fn scope(&mut self) -> raw::Isolate_Scope {
        unsafe {
            V8_Isolate_Enter(self.0);
        }
        raw::Isolate_Scope {
            isolate_: self.0,
        }
    }

    pub fn dispose(&self) {
        unsafe {
            V8_Isolate_Dispose(self.0)
        }
    }
}

impl Drop for raw::Isolate_Scope {
    fn drop(&mut self) {
        unsafe {
            V8_Isolate_Exit(self.isolate_)
        }
    }
}
