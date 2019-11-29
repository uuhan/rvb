#![allow(non_camel_case_types, unused)]
use std::mem;
use std::ffi::c_void;

pub use crate::v8::raw::Signature;
pub use crate::v8::raw::Private;
pub use crate::v8::raw::Function;
pub use crate::v8::raw::FunctionCallback;
pub use crate::v8::raw::FunctionCallbackInfo;

pub use crate::v8::raw::ConstructorBehavior;
pub use crate::v8::raw::ConstructorBehavior_kThrow;
pub use crate::v8::raw::ConstructorBehavior_kAllow;
pub use crate::v8::raw::SideEffectType;
pub use crate::v8::raw::SideEffectType_kHasSideEffect;
pub use crate::v8::raw::SideEffectType_kHasNoSideEffect;
pub use crate::v8::raw::SideEffectType_kHasSideEffectToReceiver;

use crate::v8::{
    raw,
    raw::internal::{
        Address,
    },
    prelude::*,
    Isolated,
    Template,
    TemplateTrait,
    String,
    ValueTrait,
    ObjectTemplate,
    Local,
    MaybeLocal,
    Object,
    Value,
    External,
    Data,
};

mod value;
mod template;
pub use value::*;
pub use template::*;

extern "C" {
    fn V8_FunctionCallbackInfo_GetIsolate(args: *const FunctionCallbackInfo) -> *mut raw::Isolate;
    fn V8_FunctionCallbackInfo_This(args: *const FunctionCallbackInfo) -> Local<Object>;
    fn V8_FunctionCallbackInfo_Length(args: *const FunctionCallbackInfo) -> usize;
    fn V8_FunctionCallbackInfo_Holder(args: *const FunctionCallbackInfo) -> Local<Object>;
    fn V8_FunctionCallbackInfo_NewTarget(args: *const FunctionCallbackInfo) -> Local<Value>;
    fn V8_FunctionCallbackInfo_IsConstructorCall(args: *const FunctionCallbackInfo) -> bool;
    fn V8_FunctionCallbackInfo_Data(args: *const FunctionCallbackInfo) -> Local<Value>;
    fn V8_FunctionCallbackInfo_GetReturnValue(args: *const FunctionCallbackInfo, out: *mut ReturnValue);
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
            let mut value = std::mem::MaybeUninit::<ReturnValue>::uninit();
            V8_FunctionCallbackInfo_GetReturnValue(self, value.as_mut_ptr());
            value.assume_init()
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

/// trampoline function for
extern fn callback_data_wrapper(data: *mut c_void) {
    unsafe {
        let closure: &mut Box<dyn FnMut()> = mem::transmute(data);
        closure()
    }
}

extern fn function_template(info: *const FunctionCallbackInfo) {
    unsafe {
        let args = &*info;
        let external = V8External::from(args.data());
        let external_ptr = external.value();
        let ref mut rv = args.get_return_value();

        let closure: &mut Box<dyn FnMut(*const FunctionCallbackInfo, &mut ReturnValue)>
            = mem::transmute(external_ptr);
        closure(args, rv);
    }
}

impl V8Function {
    /// Create a function in the current execution context
    /// for a given FunctionCallback.
    #[inline]
    pub fn RawNew(
        context: V8Context,
        callback: raw::FunctionCallback,
        data: V8Value,
        length: i32,
        behavior: ConstructorBehavior,
        side_effect_type: SideEffectType
    ) -> MaybeLocal<Function> {
        unsafe {
            Function::New(context, callback, data, length, behavior, side_effect_type)
        }
    }

    #[inline]
    pub fn New(
        context: V8Context,
    ) -> Option<Self> {
        let function = unsafe {
            Function::New(context, None, V8Value::Empty(), 0, ConstructorBehavior_kAllow, SideEffectType_kHasSideEffect)
        };
        function.to_local_checked().ok()
    }

    #[inline]
    pub fn Fn<F>(context: V8Context, closure: F) -> Option<Self>
        where F: FnMut(&FunctionCallbackInfo, &mut ReturnValue)
    {
        let closure: Box<Box<dyn FnMut(&FunctionCallbackInfo, &mut ReturnValue)>>
            = Box::new(Box::new(closure));
        let data = V8External::New(Box::into_raw(closure) as *mut c_void);

        unsafe {
            Function::New(
                context,
                Some(function_template),
                data.into(),
                0,
                ConstructorBehavior_kAllow,
                SideEffectType_kHasSideEffect,
            ).to_local_checked().ok()
        }
    }
}

inherit!(Function, Object, Value, Data);
inherit_local!(Function, Object, Value, Data);

impl ValueTrait for Function {}
