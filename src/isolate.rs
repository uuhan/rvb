use crate::v8::raw;
use crate::v8::Rooted;
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

    pub fn current() -> Self {
        unsafe {
            mem::transmute(raw::Isolate_GetCurrent())
        }
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

deref!(Isolate);

impl Rooted for Isolate {
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
