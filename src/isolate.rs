use std::mem;

use crate::v8::raw;
use crate::v8::Rooted;
use crate::v8::Isolated;
use crate::v8::Context;
use crate::v8::Local;
use crate::v8::Value;
use crate::v8::HandleScope;

pub use raw::Locker;
pub use raw::Unlocker;

pub const ISOLATE_DATA_SLOT: u32 = 0;

pub struct IsolateData {
    pub count: usize,
}

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
        if isolate.is_null() {
            panic!("create isolate failed");
        } else {
            unsafe {
                let init_data_ptr = Box::into_raw(Box::new(IsolateData { count: 1 }));
                V8_Isolate_SetData(isolate, ISOLATE_DATA_SLOT, init_data_ptr as *mut std::ffi::c_void);
            }
            Self(isolate)
        }
    }

    #[inline]
    pub fn Current() -> Self {
        unsafe {
            mem::transmute(raw::Isolate_GetCurrent())
        }
    }

    #[inline]
    pub fn in_context(&mut self) -> bool {
        unsafe {
            self.InContext()
        }
    }

    #[inline]
    pub fn get_current_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetCurrentContext()
        }
    }

    #[inline]
    pub fn get_entered_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetEnteredContext()
        }
    }

    #[inline]
    pub fn get_entered_or_microtask_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetEnteredOrMicrotaskContext()
        }
    }

    #[inline]
    pub fn get_incumbent_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetIncumbentContext()
        }
    }

    #[inline]
    pub fn throw_exception(&mut self, exception: Local<Value>) -> Local<Value> {
        unsafe {
            self.ThrowException(exception)
        }
    }

    #[inline]
    pub fn get_data_ptr<T>(&self, slot: u32) -> *mut T {
        unsafe {
            V8_Isolate_GetData(self.0, slot) as *mut T
        }
    }

    #[inline]
    pub fn get_data<T>(&self, slot: u32) -> &mut T {
        unsafe {
            let ptr = self.get_data_ptr(slot) as *mut T;
            match ptr.as_mut() {
                Some(ptr) => ptr,
                None => panic!(format!("Isolate::GetData with slot: {} got nothing.", slot)),
            }
        }
    }

    #[inline]
    pub fn set_data<T>(&mut self, slot: u32, data: T) {
        unsafe {
            let data_ptr = Box::into_raw(Box::new(data));
            V8_Isolate_SetData(self.0, slot, data_ptr as *mut std::ffi::c_void);
        }
    }

    #[inline]
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

    #[inline]
    pub fn set_idle(&mut self, is_idle: bool) {
        unsafe {
            self.SetIdle(is_idle)
        }
    }

    #[inline]
    pub fn is_dead(&mut self) -> bool {
        unsafe {
            self.IsDead()
        }
    }

    #[inline]
    pub fn is_in_use(&mut self) -> bool {
        unsafe {
            self.IsInUse()
        }
    }

    #[inline]
    pub fn get_array_buffer_allocator(&mut self) -> *mut raw::ArrayBuffer_Allocator {
        unsafe {
            self.GetArrayBufferAllocator()
        }
    }

    #[inline]
    pub fn dispose(&mut self) {
        unsafe {
            self.Dispose()
        }
    }
}

deref_mut!(Isolate);

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

/// Clone an Isolate
impl Clone for Isolate {
    fn clone(&self) -> Isolate {
        self.get_data::<IsolateData>(ISOLATE_DATA_SLOT).count += 1;
        Isolate(self.0)
    }
}

/// Drop an Isolate
impl Drop for Isolate {
    fn drop(&mut self) {
        let ref mut data = self.get_data::<IsolateData>(ISOLATE_DATA_SLOT);
        data.count -= 1;

        if data.count == 0 {
            self.dispose();
        }
    }
}
