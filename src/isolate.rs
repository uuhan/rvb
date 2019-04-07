#![allow(dead_code)]
use std::mem;

use crate::v8::{
    raw,
    raw::internal::{
        Address,
        Internals_kUndefinedValueRootIndex,
    },
    prelude::*,
    Rooted,
    Isolated,
    Local,
    Value,
    Primitive,
    Context,
    HandleScope,
    utils,
};

extern {
    fn V8_Isolate_Locker(isolate: *const raw::Isolate, locker: &mut Locker);
    fn V8_Isolate_Unlocker(isolate: *const raw::Isolate, unlocker: &mut Unlocker);
}

pub const ISOLATE_DATA_SLOT: u32 = 0;

pub struct IsolateData {
    pub count: usize,
}

#[must_use]
#[repr(C)]
pub struct Locker(*mut Locker);
impl Locker {
    pub fn New() -> Self {
        unsafe {
            let mut locker = mem::uninitialized();
            let isolate = raw::Isolate::GetCurrent();
            V8_Isolate_Locker(isolate, &mut locker);
            locker
        }
    }
}

#[must_use]
#[repr(C)]
pub struct Unlocker(*mut Unlocker);
impl Unlocker {
    pub fn New() -> Self {
        unsafe {
            let mut unlocker = mem::uninitialized();
            let isolate = raw::Isolate::GetCurrent();
            V8_Isolate_Unlocker(isolate, &mut unlocker);
            unlocker
        }
    }
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

    /// Returns the entered isolate for the current thread or NULL in
    /// case there is no current isolate.
    ///
    /// If it is not convenient to pass the reference of Isolate, you can use
    /// this static function.
    #[inline]
    pub fn Current() -> Self {
        unsafe {
            let isolate: Self = mem::transmute(raw::Isolate_GetCurrent());
            let ref mut data = isolate.get_data::<IsolateData>(ISOLATE_DATA_SLOT);
            data.count += 1;
            isolate
        }
    }

    /// Get the reference of raw Isolate.
    #[inline]
    pub fn current(&self) -> &mut raw::Isolate {
        unsafe {
            self.0.as_mut().unwrap()
        }
    }

    /// Returns true if this isolate has a current context.
    #[inline]
    pub fn in_context(&mut self) -> bool {
        unsafe {
            self.InContext()
        }
    }

    /// Returns the context of the currently running JavaScript, or the context
    /// on the top of the stack if no JavaScript is running.
    #[inline]
    pub fn get_current_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetCurrentContext()
        }
    }

    /// Returned the entered context.
    #[inline]
    pub fn get_entered_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetEnteredContext()
        }
    }

    /// Returns either the last context entered through V8's C++ API, or the
    /// context of the currently running microtask while processing microtasks.
    /// If a context is entered while executing a microtask, that context is
    /// returned.
    #[inline]
    pub fn get_entered_or_microtask_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetEnteredOrMicrotaskContext()
        }
    }

    /// Returns the Context that corresponds to the Incumbent realm in HTML spec.
    /// https://html.spec.whatwg.org/multipgage/webappapis.html#incumbent
    #[inline]
    pub fn get_incumbent_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetIncumbentContext()
        }
    }

    /// Schedules an exception to be thrown when returning to JavaScript. When an
    /// exception has been scheduled it is illegal to invoke any JavaScript
    /// operation; the caller must return immediately and only after the exception
    /// has been handled does it become legal to invoke JavaScript operation.
    #[inline]
    pub fn throw_exception(&mut self, exception: Local<Value>) -> Local<Value> {
        unsafe {
            self.ThrowException(exception)
        }
    }

    /// Get data ptr in slot.
    #[inline]
    pub fn get_data_ptr<T>(&self, slot: u32) -> *mut T {
        unsafe {
            V8_Isolate_GetData(self.0, slot) as *mut T
        }
    }

    /// Get data in slot.
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

    /// Set data to slot.
    #[inline]
    pub fn set_data<T>(&mut self, slot: u32, data: T) {
        unsafe {
            let data_ptr = Box::into_raw(Box::new(data));
            V8_Isolate_SetData(self.0, slot, data_ptr as *mut std::ffi::c_void);
        }
    }

    /// Drop data in slot.
    #[inline]
    pub fn drop_data<T>(&mut self, slot: u32) {
        unsafe {
            drop(Box::from_raw(self.get_data_ptr::<T>(slot)))
        }
    }

    /// Helper function to execute.
    pub fn exec<U, F>(&mut self, run: F) -> V8Result<U>
        where F: FnOnce(Local<Context>) -> V8Result<U>
        {
            self.enter();
            let _handle_scole = HandleScope::New();
            let mut context = V8Context::Default();
            context.enter();

            let result = run(context)?;

            context.exit();
            self.exit();

            Ok(result)
        }

    /// Tells the VM whether the embedder is idle or not.
    #[inline]
    pub fn set_idle(&mut self, is_idle: bool) {
        unsafe {
            self.SetIdle(is_idle)
        }
    }

    /// Check if V8 is dead and thereforce unusable. This is the case after
    /// fatal errors such as out-of-meomory situations.
    #[inline]
    pub fn is_dead(&mut self) -> bool {
        unsafe {
            self.IsDead()
        }
    }

    /// Check if this isolate is in use.
    /// True if at least one thread Enter'ed this isolate.
    #[inline]
    pub fn is_in_use(&mut self) -> bool {
        unsafe {
            self.IsInUse()
        }
    }

    /// Returns the ArrayBuffer::Allocator used in this isolate.
    #[inline]
    pub fn get_array_buffer_allocator(&mut self) -> *mut raw::ArrayBuffer_Allocator {
        unsafe {
            self.GetArrayBufferAllocator()
        }
    }

    /// Disposes the isolate. The isolate must not be entered by any
    /// thread to be disposable.
    #[inline]
    pub fn dispose(&mut self) {
        unsafe {
            self.Dispose()
        }
    }

    // Internals::GetRoot implementation
    #[inline]
    pub fn get_root(&self, index: i32) -> *mut Address {
        utils::get_root(self.current(), index)
    }

    /// v8::Undefined(Isolate* isolate)
    #[inline]
    pub fn undefined(&self) -> Local<Primitive> {
        let slot = self.get_root(Internals_kUndefinedValueRootIndex);
        unsafe {
            mem::transmute(slot)
        }
    }

    /// TODO: Lock this Isolate
    pub fn lock(&mut self) -> &mut Self {
        self
    }

    /// TODO: Unlock this Isolate
    pub fn unlock(&mut self) -> &mut Self {
        self
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
