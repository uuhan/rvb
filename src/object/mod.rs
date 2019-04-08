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
    prelude::*,
    Local,
    Value,
    Data,
    String,
    Context,
    ValueTrait,
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
    #[inline]
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            Object::New(isolate)
        }
    }

    #[inline]
    pub fn set<K: Into<Local<Value>>, V: Into<Local<Value>>>(&mut self, key: K, value: V) -> bool {
        unsafe {
            self.Set(key.into(), value.into())
        }
    }

    #[inline]
    pub fn get<K: Into<Local<Value>>>(&mut self, key: K) -> Local<Value> {
        unsafe {
            self.Get(key.into())
        }
    }

    #[inline]
    pub fn create_data_property(&mut self,
                                context: V8Context,
                                key: V8Name,
                                value: V8Value) -> Maybe<bool> {
        unsafe {
            self.CreateDataProperty(context, key ,value)
        }
    }

    /// Sets the value in an internal field.
    #[inline]
    pub fn set_internal_field(&mut self, index: u32, value: Local<Value>) {
        unsafe {
            self.SetInternalField(index as ::std::os::raw::c_int, value)
        }
    }

    /// Gets the value from an internal field.
    #[inline]
    pub fn get_internal_field(&mut self, index: u32) {
        unimplemented!()
    }
}

inherit!(Object, Value, Data);
inherit_local!(Object, Value, Data);

impl ValueTrait for Object {}
