use crate::v8::raw;

extern "C" {
    pub fn V8_Initialize_Platform() -> Platform;
}

type RawPlatform = *mut std::ffi::c_void;

#[repr(C)]
pub struct Platform(RawPlatform);

impl Platform {
    pub fn New() -> Self {
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
