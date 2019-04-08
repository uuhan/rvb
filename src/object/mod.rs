use std::ptr;
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
    internal::Address,
    Object,
    AccessorGetterCallback,
    AccessorSetterCallback,
    AccessControl,
    AccessorSignature,
    FunctionCallback,
    SideEffectType,
    NamedPropertyHandlerConfiguration,

    PropertyDescriptor,

    PropertyAttribute,
    PropertyAttribute_None,
    PropertyAttribute_ReadOnly,
    PropertyAttribute_DontEnum,
    PropertyAttribute_DontDelete,
};

extern {
    fn V8_Object_GetInternalField(obj: &mut Object, index: u32, out: *mut V8Value);
}

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

    /// Get the prototype object. This does not skip objects marked to
    /// be skipped by __proto__ and it does not consult the security
    /// handler.
    #[inline]
    pub fn get_prototype(&mut self) -> V8Value {
        unsafe {
            self.GetPrototype()
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

    #[inline]
    pub fn define_own_property(&mut self,
                               context: V8Context,
                               key: V8Name,
                               value: V8Value,
                               attributes: PropertyAttribute) -> Maybe<bool> {
        unsafe {
            self.DefineOwnProperty(context, key, value, attributes)
        }
    }

    #[inline]
    pub fn define_property(&mut self,
                           context: V8Context,
                           key: V8Name,
                           descriptor: &mut PropertyDescriptor) -> Maybe<bool> {
        unsafe {
            self.DefineProperty(context, key, descriptor)
        }
    }

    #[inline]
    pub fn internal_field_count(&mut self) -> u32 {
        unsafe {
            self.InternalFieldCount() as u32
        }
    }

    /// Sets the value in an internal field.
    #[inline]
    pub fn set_internal_field<T: Clone + Into<V8Value>>(&mut self, index: u32, value: T) {
        unsafe {
            self.SetInternalField(index as ::std::os::raw::c_int, value.into())
        }
    }

    /// Gets the value from an internal field.
    #[inline]
    pub fn get_internal_field<T: From<V8Value>>(&mut self, index: u32) -> Option<T> {
        unsafe {
            let field = ptr::null_mut();
            V8_Object_GetInternalField(self, index, field);
            field.as_mut::<'static>().map(|v| T::from(*v))
        }
    }
}

inherit!(Object, Value, Data);
inherit_local!(Object, Value, Data);

impl ValueTrait for Object {}
