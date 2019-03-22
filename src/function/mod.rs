#![allow(non_camel_case_types, unused)]
pub use crate::v8::raw::Signature;
pub use crate::v8::raw::Function;
pub use crate::v8::raw::FunctionTemplate;
pub use crate::v8::raw::FunctionCallback;
pub use crate::v8::raw::FunctionCallbackInfo;

pub use crate::v8::raw::ConstructorBehavior_kThrow;
pub use crate::v8::raw::ConstructorBehavior_kAllow;
pub use crate::v8::raw::SideEffectType_kHasSideEffect;
pub use crate::v8::raw::SideEffectType_kHasNoSideEffect;
pub use crate::v8::raw::SideEffectType_kHasSideEffectToReceiver;

use crate::v8::{
    Isolated,
    V8Template,
    Local,
    MaybeLocal,
    Context,
    Data,
    Value,
};

// static kHolderIndex: i8 = 0;
// static kIsolateIndex: i8 = 1;
// static kReturnValueDefaultValueIndex: i8 = 2;
// static kReturnValueIndex: i8 = 3;
// static kDataIndex: i8 = 4;
// static kNewTargetIndex: i8 = 5;

impl FunctionCallbackInfo {
    pub fn isolate(&self) {
        unimplemented!()
    }
}

impl<'a> Isolated<'a> for FunctionTemplate {}
impl V8Template for FunctionTemplate {}

impl Local<FunctionTemplate> {
    /// Create a function template.
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            FunctionTemplate::New(
                isolate,
                None,
                Local::<Value>::Empty(),
                Local::<Signature>::Empty(),
                0,
                ConstructorBehavior_kAllow,
                SideEffectType_kHasSideEffect,
                )
        }
    }

    /// Set the call-handler callback for a FunctionTemplate.
    /// This callback is called whenever the function created from this
    /// FunctionTemplate is called.
    pub fn set_handler(&mut self, handler: FunctionCallback) -> &mut Self {
        unsafe {
            self.SetCallHandler(handler, Local::<Value>::Empty(), SideEffectType_kHasSideEffect)
        }
        self
    }

    /// Returns the unique function instance in the current execution context.
    pub fn get_function(&mut self, context: Local<Context>) -> MaybeLocal<Function> {
        unsafe {
            self.GetFunction(context)
        }
    }
}

impl Local<Function> {
    pub fn New() -> Self {
        unimplemented!()
    }
}

inherit_local!(FunctionTemplate, Data);
inherit_local!(Function, Data);
