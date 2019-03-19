mod array;
mod map;
mod set;

pub use array::*;
pub use map::*;
pub use set::*;

pub use crate::v8::{
    Local,
    Value,
    FunctionTemplate,
};

pub use crate::v8::raw::{
    self,
    Object,
    ObjectTemplate,
};

use crate::v8::Isolated;

impl Local<ObjectTemplate> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            ObjectTemplate::New(isolate.0, Local::<FunctionTemplate>::Empty())
        }
    }
}

impl Local<Object> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            Object::New(isolate.0)
        }
    }

    pub fn set(&mut self, key: Local<Value>, value: Local<Value>) -> bool {
        unsafe {
            self.Set(key, value)
        }
    }

    pub fn get(&mut self, key: Local<Value>) -> Local<Value> {
        unsafe {
            self.Get(key)
        }
    }
}
