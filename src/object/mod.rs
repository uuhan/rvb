mod array;
mod map;
mod set;
mod promise;
mod proxy;
mod date;
mod regexp;

mod template;

pub use array::*;
pub use map::*;
pub use set::*;
pub use promise::*;
pub use proxy::*;
pub use date::*;
pub use regexp::*;
pub use template::*;

use crate::v8::{
    Local,
    MaybeLocal,
    Value,
    Data,
    Context,
    TemplateTrait,
    ValueTrait,
    String,
    Template,
    FunctionTemplate,
    Isolated,
};

pub use crate::v8::raw::{
    Object,
    AccessorGetterCallback,
    AccessorSetterCallback,
    AccessControl,
    PropertyAttribute,
    AccessorSignature,
    FunctionCallback,
    SideEffectType,
    NamedPropertyHandlerConfiguration,
};

impl Local<Object> {
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            Object::New(isolate)
        }
    }

    pub fn set<K: Into<Local<Value>>, V: Into<Local<Value>>>(&mut self, key: K, value: V) -> bool {
        unsafe {
            self.Set(key.into(), value.into())
        }
    }

    pub fn get<K: Into<Local<Value>>>(&mut self, key: K) -> Local<Value> {
        unsafe {
            self.Get(key.into())
        }
    }
}

inherit!(Object, Value, Data);
inherit_local!(Object, Value, Data);

impl ValueTrait for Object {}
