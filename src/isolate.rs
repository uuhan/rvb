use crate::raw;
use crate::Root;
use std::mem;
use std::ops::DerefMut;
use std::ops::Deref;

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
            self.Enter();
        }
        raw::Isolate_Scope {
            isolate_: self.0,
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

impl Deref for Isolate {
    type Target = raw::Isolate;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for Isolate {
    fn deref_mut(&mut self) -> &mut raw::Isolate {
        unsafe {
            &mut *self.0
        }
    }
}

impl Root for Isolate {
    unsafe fn allocate() -> Self {
        Isolate::new()
    }

    unsafe fn enter(&mut self) {
        self.Enter()
    }

    unsafe fn exit(&mut self) {
        self.Exit()
    }
}
