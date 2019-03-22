use std::mem;

use crate::v8::raw;
use crate::v8::Rooted;
use crate::v8::Isolated;
use crate::v8::Context;
use crate::v8::Local;
use crate::v8::HandleScope;

pub use raw::Locker;
pub use raw::Unlocker;
pub type IsolateData = *mut std::ffi::c_void;
#[repr(C)]
pub struct Isolate(pub *mut raw::Isolate);

extern "C" {
    fn V8_Isolate_SetData(isolate: *mut raw::Isolate, slot: u32, data: IsolateData);
    fn V8_Isolate_GetData(isolate: *mut raw::Isolate, slot: u32) -> IsolateData;
}

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

    pub fn get(&mut self, slot: u32, data: IsolateData) {
        unsafe {
            V8_Isolate_SetData(self.0, slot, data);
        }
    }

    pub fn set(&mut self, slot: u32) -> IsolateData {
        unsafe {
            V8_Isolate_GetData(self.0, slot)
        }
    }

    pub fn exec<U, F>(&mut self, run: F) -> U
        where F: FnOnce(Local<Context>) -> U
        {
            self.enter();
            let _handle_scole = HandleScope::New();
            let mut context = Local::<Context>::Default();
            context.enter();

            let result = run(context);

            context.exit();
            self.exit();

            result
        }

    pub fn dispose(&mut self) {
        unsafe {
            self.Dispose()
        }
    }
}

deref!(Isolate);

impl Rooted for Isolate {
    fn allocate() -> Self {
        Isolate::New()
    }

    fn enter(&mut self) {
        unsafe {
            self.Enter()
        }
    }

    fn exit(&mut self) {
        unsafe {
            self.Exit()
        }
    }
}
