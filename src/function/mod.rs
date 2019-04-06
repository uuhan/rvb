#![allow(non_camel_case_types, unused)]
use std::mem;
use std::ffi::c_void;

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
    raw::internal::{
        Address,
    },
    Isolated,
    Template,
    V8Template,
    V8String,
    V8Value,
    ObjectTemplate,
    Local,
    MaybeLocal,
    Context,
    Object,
    Value,
    External,
    Data,
};

mod value;
use value::ReturnValue;

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

    /// Get arguments[index]
    #[inline]
    pub fn at(&self, index: u32) -> Local<Value> {
        unsafe {
            let target = self.values_.offset(-(index as isize));
            if target.is_null() {
                Local::<Value>::Empty()
            } else {
                mem::transmute(target)
            }
        }
    }
}

impl<'a> Isolated<'a> for FunctionTemplate {}
impl V8Template for FunctionTemplate {}

#[inline]
extern fn function_template(info: *const FunctionCallbackInfo) {
    unsafe {
        let args = &*info;
        let external = Local::<External>::from(args.data());
        let external_ptr = external.Value();
        let ref mut rv = args.get_return_value();

        let closure: &mut Box<FnMut(*const FunctionCallbackInfo, &mut ReturnValue)>
            = mem::transmute(external_ptr);
        closure(args, rv);
    }
}

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

    /// Create a function template.
    #[inline]
    pub fn Call<F>(callback: F) -> Self
        where F: FnMut(&FunctionCallbackInfo, &mut ReturnValue)
        {
            let isolate = Self::GetIsolate();
            let callback: Box<Box<FnMut(&FunctionCallbackInfo, &mut ReturnValue)>>
                = Box::new(Box::new(callback));
            let data = Local::<External>::New(Box::into_raw(callback) as *mut c_void);
            unsafe {
                FunctionTemplate::New(
                    isolate,
                    Some(function_template),
                    data.into(),
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
                Some(data) => self.SetCallHandler(handler, data.into(), SideEffectType_kHasSideEffect),
                None => self.SetCallHandler(handler, Local::<Value>::Empty(), SideEffectType_kHasSideEffect),
            }
        }
        self
    }

    /// Set the callback for a FunctionTemplate by closure.
    #[inline]
    pub fn set_call_closure<F>(&mut self, callback: F)
        where F: FnMut(&FunctionCallbackInfo, &mut ReturnValue)
        {
            let callback: Box<Box<FnMut(&FunctionCallbackInfo, &mut ReturnValue)>>
                = Box::new(Box::new(callback));
            let data = Local::<External>::New(Box::into_raw(callback) as *mut c_void);
            self.set_call_handler(Some(function_template), Some(data.into()));
        }

    /// Returns the unique function instance in the current execution context.
    #[inline]
    pub fn get_function(&mut self, context: Local<Context>) -> MaybeLocal<Function> {
        unsafe {
            self.GetFunction(context)
        }
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

    /// A PrototypeProviderTemplate is another function template whose prototype
    /// property is used for this template. This is manually exclusive with setting
    /// a prototype template indirectly by calling PrototypeTemplate() or using
    /// Inherit().
    #[inline]
    pub fn set_property_provider_template(&mut self, prototype_provider: Local<FunctionTemplate>) {
        unsafe {
            self.SetPrototypeProviderTemplate(prototype_provider)
        }
    }

    /// When set to true, no access check will be performed on the receiver of a
    /// function call. Currently defaults to true, but this is subject to change.
    #[inline]
    pub fn set_accept_any_receiver(&mut self, value: bool) {
        unsafe {
            self.SetAcceptAnyReceiver(value)
        }
    }
}

impl Local<Function> {
    pub fn New() -> Self {
        unimplemented!()
    }
}

inherit!(FunctionTemplate, Template, Data);
inherit_local!(FunctionTemplate, Template, Data);

inherit!(Function, Object, Value, Data);
inherit_local!(Function, Object, Value, Data);

impl V8Value for Function {}
impl V8Value for FunctionTemplate {}
