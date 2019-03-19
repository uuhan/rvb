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
    Local,
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

impl Isolated for FunctionTemplate {}

impl Local<FunctionTemplate> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            FunctionTemplate::New(
                isolate.0,
                None,
                Local::<Value>::Empty(),
                Local::<Signature>::Empty(),
                0,
                ConstructorBehavior_kAllow,
                SideEffectType_kHasSideEffect,
                )
        }
    }
}

impl Local<Function> {
    pub fn New() -> Self {
        unimplemented!()
    }
}
