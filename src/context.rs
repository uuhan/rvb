use std::ptr;

use crate::v8::raw;
use crate::v8::Local;
use crate::v8::Isolated;
use crate::v8::Rooted;
use crate::v8::Object;
use crate::v8::ObjectTemplate;
use crate::v8::MaybeLocal;
use crate::v8::Value;
use crate::v8::DeserializeInternalFieldsCallback;

pub use raw::Context;

/// params to use to create a context
pub struct ContextParams {
    pub isolate: *mut raw::Isolate,
    pub extensions: *mut raw::ExtensionConfiguration,
    pub global_template: MaybeLocal<ObjectTemplate>,
    pub global_object: MaybeLocal<Value>,
    pub internal_fields_deserializer: DeserializeInternalFieldsCallback,
    pub microtask_queue: *mut raw::MicrotaskQueue,
}

impl Default for ContextParams {
    /// default context params
    fn default() -> Self {
        let isolate = unsafe { raw::Isolate::GetCurrent() };
        ContextParams {
            isolate: isolate,
            extensions: ptr::null_mut(),
            global_template: MaybeLocal::<ObjectTemplate>::Empty(),
            global_object: MaybeLocal::<Value>::Empty(),
            internal_fields_deserializer: DeserializeInternalFieldsCallback::default(),
            microtask_queue: ptr::null_mut(),
        }
    }
}

impl Local<Context> {
    pub fn New(params: ContextParams) -> Self {
        let ContextParams {
            isolate,
            extensions,
            global_template,
            global_object,
            internal_fields_deserializer,
            microtask_queue,
        } = params;
        unsafe {
            Context::New(
                isolate,
                extensions,
                global_template,
                global_object,
                internal_fields_deserializer,
                microtask_queue)
        }
    }

    pub fn Default() -> Self {
        let isolate = Self::GetIsolate();
        let mut params = ContextParams::default();
        params.isolate = isolate;
        Local::<Context>::New(params)
    }

    /// Returns the global proxy object.
    ///
    /// Global proxy object is a thin wrapper those prototype points to actual
    /// context\'s glbal object with the properties like Object, etc. This is down
    /// that way for security reasons (for more details see
    /// https://wiki.mozilla.org/Gecko:SplitWindow).
    ///
    /// Please note that changes to global proxy object prototype most probably
    /// would break VM---v8 expects only global object as a prototype of global
    /// proxy object.
    pub fn global(&mut self) -> Local<Object> {
        unsafe {
            self.Global()
        }
    }

    /// Detaches the global object from its context before
    /// the global object can be reused to create a new context.
    pub fn detach(&mut self) {
        unsafe {
            self.DetachGlobal()
        }
    }

    /// exec function inside this context
    pub fn exec<U, F>(&mut self, run: F) -> U
        where F: FnOnce(&mut Self) -> U
        {
            self.enter();
            let result = run(self);
            self.exit();
            result
        }
}

impl Rooted for Local<Context> {
    fn allocate() -> Self {
        Local::<Context>::Default()
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
