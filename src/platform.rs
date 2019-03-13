use crate::v8::raw;

extern "C" {
    pub fn V8_Initialize_Platform() -> Platform;
}

#[repr(C)]
pub struct Platform(*mut std::ffi::c_void);

impl Platform {
    pub fn new() -> Self {
        unsafe { V8_Initialize_Platform() }
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        unsafe {
            raw::V8_Dispose();
            raw::V8_ShutdownPlatform();
        }
    }
}
