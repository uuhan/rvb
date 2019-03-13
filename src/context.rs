use crate::raw;
use crate::Isolate;
use std::ops::Deref;
use std::ops::DerefMut;

extern "C" {
    pub fn V8_Context_New(isolate: *mut raw::Isolate) -> raw::Local<raw::Context>;
    pub fn V8_Context_Enter(context: raw::Local<raw::Context>);
    pub fn V8_Context_Exit(context: raw::Local<raw::Context>);
}

#[repr(C)]
pub struct Context(pub raw::Local<raw::Context>);

impl Context {
    pub fn new(isolate: &mut Isolate) -> Self {
        let ctx = unsafe {
            V8_Context_New(isolate.0)
        };

        Self(ctx)
    }

    pub fn enter(&self) -> &Self {
        unsafe {
            V8_Context_Enter(self.0)
        }
        self
    }

    pub fn exit(&self) -> &Self {
        unsafe {
            V8_Context_Exit(self.0);
        }
        self
    }
}

impl Deref for Context {
    type Target = raw::Context;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0.val_
        }
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut raw::Context {
        unsafe {
            &mut *self.0.val_
        }
    }
}
