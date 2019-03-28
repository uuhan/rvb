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
    V8String,
    ObjectTemplate,
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
    #[inline]
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
    #[inline]
    pub fn set_call_handler(&mut self, handler: FunctionCallback) -> &mut Self {
        unsafe {
            self.SetCallHandler(handler, Local::<Value>::Empty(), SideEffectType_kHasSideEffect)
        }
        self
    }

    /// Set the predefined length property for the FunctionTemplate.
    #[inline]
    pub fn set_length(&mut self, length: i32) {
        unsafe {
            self.SetLength(length.into())
        }
    }

    /// Set the class name of the FunctionTemplate. This is used for
    /// printing objects created with the function created from the
    /// FunctionTemplate as its constructor.
    #[inline]
    pub fn set_class_name(&mut self, name: Local<V8String>) {
        unsafe {
            self.SetClassName(name)
        }
    }

    /// Causes the function template to inherit from a parent function template.
    /// This means the function's prototype.__proto__ is set to the parent
    /// function's prototype.
    #[inline]
    pub fn inherit(&mut self, parent: Self) {
        unsafe {
            self.Inherit(parent)
        }
    }

    /// Get the InstanceTemplate.
    #[inline]
    pub fn instance_template(&mut self) -> Local<ObjectTemplate> {
        unsafe {
            self.InstanceTemplate()
        }
    }

    /// A PrototypeTemplate is the template used to create the prototype object
    /// of the function created by this template.
    #[inline]
    pub fn prototype_template(&mut self) -> Local<ObjectTemplate> {
        unsafe {
            self.PrototypeTemplate()
        }
    }

    /// Returns the unique function instance in the current execution context.
    #[inline]
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
