use std::mem;
use std::ops::DerefMut;
use std::ops::Deref;

use crate::v8::raw;
use crate::v8::Rooted;
use crate::v8::Isolated;
use crate::v8::Context;
use crate::v8::Local;
use crate::v8::HandleScope;

#[repr(C)]
pub struct Isolate(pub *mut raw::Isolate);

impl Isolate {
    pub fn New() -> Self {
        let isolate = unsafe {
            let mut create_params: raw::Isolate_CreateParams = mem::zeroed();
            create_params.array_buffer_allocator = raw::ArrayBuffer_Allocator::NewDefaultAllocator();
            raw::Isolate::New(&create_params)
        };
        assert_eq!(false, isolate.is_null());
        Self(isolate)
    }

    pub fn Current() -> Self {
        unsafe {
            mem::transmute(raw::Isolate_GetCurrent())
        }
    }

    pub fn with<U, F>(&mut self, run: F) -> U
        where F: FnOnce(Local<Context>) -> U
        {
            unsafe {
                self.enter();
                let _handle_scole = HandleScope::New();
                let mut context = Local::<Context>::New();
                context.enter();
                let result = run(context);
                context.exit();
                self.exit();

                result
            }
        }

    pub fn dispose(&mut self) {
        unsafe {
            self.Dispose()
        }
    }
}

impl Drop for raw::Isolate_Scope {
    fn drop(&mut self) {
        unsafe {
            (*self.isolate_).Exit()
        }
    }
}

deref!(Isolate);

impl Rooted for Isolate {
    unsafe fn allocate() -> Self {
        Isolate::New()
    }

    unsafe fn enter(&mut self) {
        self.Enter()
    }

    unsafe fn exit(&mut self) {
        self.Exit()
    }
}
