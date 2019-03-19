use std::mem;

mod array;
mod map;
mod set;

pub use array::*;
pub use map::*;
pub use set::*;

use crate::v8::{
    Local,
    Value,
    Name,
    Data,
    Template,
    FunctionTemplate,
    Isolated,

    PropertyAttribute_None,
};

pub use crate::v8::raw::{
    self,
    Object,
    ObjectTemplate,
};

extern {
    fn V8_Template_Set(obj: Local<ObjectTemplate>, name: Local<Name>, value: Local<Data>);
}

impl Local<ObjectTemplate> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            ObjectTemplate::New(isolate.0, Local::<FunctionTemplate>::Empty())
        }
    }


    pub fn set(self, name: Local<Name>, value: Local<Data>) {
        unsafe {
            V8_Template_Set(self, name, value);
            // let mut self_ =
            //     mem::transmute::<ObjectTemplate, Template>(*self.val_);
            // self_.Set(
            //     name,
            //     value,
            //     PropertyAttribute_None)
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

// impl V8Template for ObjectTemplate {}
