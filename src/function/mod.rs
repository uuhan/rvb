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
    raw,
    raw::internal::Address,
    Isolated,
    V8Template,
    V8String,
    Object,
    ObjectTemplate,
    Local,
    MaybeLocal,
    Context,
    Data,
    Value,
};

extern "C" {
    fn V8_FunctionCallbackInfo_GetIsolate(args: *const FunctionCallbackInfo) -> *mut raw::Isolate;
    fn V8_FunctionCallbackInfo_This(args: *const FunctionCallbackInfo) -> Local<Object>;
    fn V8_FunctionCallbackInfo_Length(args: *const FunctionCallbackInfo) -> usize;
    fn V8_FunctionCallbackInfo_Holder(args: *const FunctionCallbackInfo) -> Local<Object>;
    fn V8_FunctionCallbackInfo_NewTarget(args: *const FunctionCallbackInfo) -> Local<Value>;
    fn V8_FunctionCallbackInfo_IsConstructorCall(args: *const FunctionCallbackInfo) -> bool;
    fn V8_FunctionCallbackInfo_Data(args: *const FunctionCallbackInfo) -> Local<Value>;
    fn V8_FunctionCallbackInfo_GetReturnValue(args: *const FunctionCallbackInfo, out: &mut ReturnValue);
}

#[repr(C)]
pub struct ReturnValue {
    value_: *mut Address,
}

impl ReturnValue {
    pub fn get(&self) -> Local<Value> {
        unimplemented!()
    }

    pub fn set<T>(&self, value: T) {
        unimplemented!()
    }
}

// static kHolderIndex: i8 = 0;
// static kIsolateIndex: i8 = 1;
// static kReturnValueDefaultValueIndex: i8 = 2;
// static kReturnValueIndex: i8 = 3;
// static kDataIndex: i8 = 4;
// static kNewTargetIndex: i8 = 5;

impl FunctionCallbackInfo {
    #[inline]
    pub fn get_isolate(&self) -> &mut raw::Isolate {
        let isolate = unsafe {
            V8_FunctionCallbackInfo_GetIsolate(self)
        };
        if isolate.is_null() {
            panic!("can not get isolate from FunctionCallbackInfo");
        } else {
            unsafe {
                isolate.as_mut().unwrap()
            }
        }
    }

    #[inline]
    pub fn get_return_value(&self) -> ReturnValue {
        unsafe {
            let mut value = std::mem::uninitialized();
            V8_FunctionCallbackInfo_GetReturnValue(self, &mut value);
            value
        }
    }

    #[inline]
    pub fn this(&self) -> Local<Object> {
        unsafe {
            V8_FunctionCallbackInfo_This(self)
        }
    }

    #[inline]
    pub fn data(&self) -> Local<Value> {
        unsafe {
            V8_FunctionCallbackInfo_Data(self)
        }
    }

    #[inline]
    pub fn length(&self) -> usize {
        unsafe {
            V8_FunctionCallbackInfo_Length(self)
        }
    }

    #[inline]
    pub fn holder(&self) -> Local<Object> {
        unsafe {
            V8_FunctionCallbackInfo_Holder(self)
        }
    }

    #[inline]
    pub fn new_target(&self) -> Local<Value> {
        unsafe {
            V8_FunctionCallbackInfo_NewTarget(self)
        }
    }

    #[inline]
    pub fn is_constructor_call(&self) -> bool {
        unsafe {
            V8_FunctionCallbackInfo_IsConstructorCall(self)
        }
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
    pub fn set_call_handler(&mut self, handler: FunctionCallback, data: Option<Local<Value>>) -> &mut Self {
        unsafe {
            match data {
                Some(data) => self.SetCallHandler(handler, data, SideEffectType_kHasSideEffect),
                None => self.SetCallHandler(handler, Local::<Value>::Empty(), SideEffectType_kHasSideEffect),
            }
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
