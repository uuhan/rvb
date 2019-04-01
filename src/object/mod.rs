mod array;
mod map;
mod set;
mod promise;
mod proxy;
mod date;
mod regexp;

pub use array::*;
pub use map::*;
pub use set::*;
pub use promise::*;
pub use proxy::*;
pub use date::*;
pub use regexp::*;

use crate::v8::{
    Local,
    MaybeLocal,
    Value,
    Data,
    Context,
    V8Template,
    FunctionTemplate,
    Isolated,
};

pub use crate::v8::raw::{
    self,
    Object,
    ObjectTemplate,
};

impl Local<ObjectTemplate> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            ObjectTemplate::New(isolate, Local::<FunctionTemplate>::Empty())
        }
    }

    pub fn constructor(&mut self, ctx: Local<Context>) -> MaybeLocal<Object> {
        unsafe {
            self.NewInstance(ctx)
        }
    }
}

impl Local<Object> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            Object::New(isolate)
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

inherit!(ObjectTemplate, Data);
inherit!(Object, Data);
inherit!(ObjectTemplate, Value);
inherit!(Object, Value);

inherit_local!(ObjectTemplate, Data);
inherit_local!(Object, Data);
inherit_local!(ObjectTemplate, Value);
inherit_local!(Object, Value);

impl V8Template for ObjectTemplate {}
