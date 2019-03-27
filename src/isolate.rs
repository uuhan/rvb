use std::mem;

use crate::v8::raw;
use crate::v8::Rooted;
use crate::v8::Isolated;
use crate::v8::Context;
use crate::v8::Local;
use crate::v8::HandleScope;

pub use raw::Locker;
pub use raw::Unlocker;

pub const ISOLATE_DATA_SLOT: u32 = 0;

#[repr(C)]
pub struct Isolate(pub *mut raw::Isolate);

extern "C" {
    fn V8_Isolate_SetData(isolate: *mut raw::Isolate, slot: u32, data: *mut std::ffi::c_void);
    fn V8_Isolate_GetData(isolate: *mut raw::Isolate, slot: u32) -> *mut std::ffi::c_void;
}

impl Isolate {
    pub fn New() -> Self {
        let isolate = unsafe {
            let mut create_params: raw::Isolate_CreateParams = mem::zeroed();
            create_params.array_buffer_allocator = raw::ArrayBuffer_Allocator::NewDefaultAllocator();
            raw::Isolate::New(&create_params)
        };
        assert!(!isolate.is_null());
        Self(isolate)
    }

    pub fn Current() -> Self {
        unsafe {
            mem::transmute(raw::Isolate_GetCurrent())
        }
    }

    pub fn get_data_ptr<T>(&self, slot: u32) -> *mut T {
        unsafe {
            V8_Isolate_GetData(self.0, slot) as *mut T
        }
    }

    pub fn get_data<T>(&self, slot: u32) -> &mut T {
        unsafe {
            let ptr = self.get_data_ptr(slot) as *mut T;
            match ptr.as_mut() {
                Some(ptr) => ptr,
                None => panic!(format!("Isolate::GetData with slot: {} got nothing.", slot)),
            }
        }
    }

    pub fn set_data<T>(&mut self, slot: u32, data: T) {
        unsafe {
            let data_ptr = Box::into_raw(Box::new(data));
            V8_Isolate_SetData(self.0, slot, data_ptr as *mut std::ffi::c_void);
        }
    }

    pub fn drop_data<T>(&mut self, slot: u32) {
        unsafe {
            drop(Box::from_raw(self.get_data_ptr::<T>(slot)))
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

// impl Clone for Isolate {
//     fn clone(&self) -> Isolate {
//         println!("clone");
//         self.get_data::<IsolateData>(ISOLATE_DATA_SLOT).count += 1;
//         Isolate(self.0)
//     }
// }

impl Drop for Isolate {
    fn drop(&mut self) {
        self.dispose()
    }
}
