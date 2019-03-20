use std::mem;
use crate::v8::raw;
pub use crate::v8::raw::platform::{
    MessageLoopBehavior,
    MessageLoopBehavior_kDoNotWait,
    MessageLoopBehavior_kWaitForWork,
};

extern "C" {
    pub fn V8_Initialize_Platform() -> RawPlatform;
    pub fn V8_Get_Platform(platform: RawPlatform, out: &mut &mut raw::Platform);
}

type RawPlatform = *mut std::ffi::c_void;

#[repr(C)]
pub struct Platform(pub *mut raw::Platform);

impl Platform {
    pub fn New() -> Self {
        unsafe {
            let mut platform: &mut raw::Platform = mem::uninitialized();
            let raw = V8_Initialize_Platform();
            V8_Get_Platform(raw, &mut platform);
            Platform(platform)
        }
    }

    pub fn pump_message_loop(&mut self) -> bool {
        unsafe {
            raw::platform::PumpMessageLoop(
                self.0,
                raw::Isolate::GetCurrent(),
                MessageLoopBehavior_kDoNotWait
            )
        }
    }
}

deref!(Platform);

impl Drop for Platform {
    fn drop(&mut self) {
        unsafe {
            raw::V8_Dispose();
            raw::V8_ShutdownPlatform();
        }
    }
}
