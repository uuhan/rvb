use crate::v8::raw;
use crate::v8::Local;
use crate::v8::Isolated;
use crate::v8::Rooted;

extern "C" {
    pub fn V8_Context_New(isolate: *mut raw::Isolate) -> Local<Context>;
}

pub use raw::Context;

impl Local<Context> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            V8_Context_New(isolate.0)
        }
    }
}

impl Rooted for Local<Context> {
    unsafe fn allocate() -> Self {
        Local::<Context>::New()
    }

    unsafe fn enter(&mut self) {
        (*self.val_).Enter()
    }

    unsafe fn exit(&mut self) {
        (*self.val_).Exit()
    }
}
