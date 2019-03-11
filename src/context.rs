use crate::raw;
use crate::Isolate;
use crate::Local;

extern "C" {
    pub fn V8_Context_New(isolate: *mut raw::Isolate) -> Local<raw::Context>;
    pub fn V8_Context_Enter(context: *mut raw::Context);
    pub fn V8_Context_Exit(context: *mut raw::Context);
}
#[repr(C)]
pub struct Context(pub Local<raw::Context>);

impl Context {
    pub fn new(isolate: &mut Isolate) -> Self {
        let ctx = unsafe {
            V8_Context_New(isolate.0)
        };

        Self(ctx)
    }

    pub fn enter(&self) -> &Self {
        unsafe {
            V8_Context_Enter(self.0.val_)
        }
        self
    }

    pub fn exit(&self) -> &Self {
        unsafe {
            V8_Context_Exit(self.0.val_);
        }
        self
    }
}
