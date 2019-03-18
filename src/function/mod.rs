#![allow(non_camel_case_types, unused)]
pub use crate::v8::raw::Function;
pub use crate::v8::raw::FunctionTemplate;
pub use crate::v8::raw::FunctionCallback;
pub use crate::v8::raw::FunctionCallbackInfo;

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
