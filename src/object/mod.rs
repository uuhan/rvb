use std::mem;

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

use cfg_if::cfg_if;
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
    AccessorNameGetterCallback,
    AccessorNameSetterCallback,
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
    fn V8_Object_GetInternalField(obj: &mut V8Object, index: u32, out: *mut V8Value);
}

impl Local<Object> {
    #[inline]
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            Object::New(isolate)
        }
    }

    cfg_if! {
        if #[cfg(any(feature = "7_4_0", feature = "7_5_0", feature = "7_6_0"))] {
            #[inline]
            pub fn set<K: Into<Local<Value>>, V: Into<Local<Value>>>(&mut self, key: K, value: V) -> bool
            {
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
        } else if #[cfg(feature = "7_8_0")] {
            #[inline]
            pub fn set<K: Into<Local<Value>>, V: Into<Local<Value>>>(&mut self, ctx: V8Context, key: K, value: V) -> Maybe<bool>
            {
                unsafe {
                    self.Set(ctx, key.into(), value.into())
                }
            }
            #[inline]
            pub fn get<K: Into<Local<Value>>>(&mut self, ctx: V8Context, key: K) -> MaybeLocal<Value> {
                unsafe {
                    self.Get(ctx, key.into())
                }
            }
        } else {
            #[inline]
            pub fn set<K: Into<Local<Value>>, V: Into<Local<Value>>>(&mut self, ctx: V8Context, key: K, value: V) -> Maybe<bool>
            {
                unsafe {
                    self.Set(ctx, key.into(), value.into())
                }
            }
            #[inline]
            pub fn get<K: Into<Local<Value>>>(&mut self, ctx: V8Context, key: K) -> MaybeLocal<Value> {
                unsafe {
                    self.Get(ctx, key.into())
                }
            }
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
    pub fn get_internal_field<T: From<V8Value>>(&mut self, index: u32) -> T {
        unsafe {
            let mut field = mem::uninitialized();
            V8_Object_GetInternalField(self, index, &mut field);
            T::from(field)
        }
    }

    /// Gets the property attributes of a property which can be None or
    /// any combination of ReadOnly, DontEnum and DontDelete. Returns
    /// None when the property doesn't exist.
    #[inline]
    pub fn get_property_attributes(&mut self, context: V8Context, key: V8Value) -> Maybe<PropertyAttribute> {
        unsafe {
            self.GetPropertyAttributes(context, key)
        }
    }

    /// Object::Has() calls the abstract operation HasProperty(O, P) described
    /// in ECMA-262, 7.3.10. Has() returns true, if the object has the property,
    /// either own or on the prototype chain.  Interceptors, i.e.,
    /// PropertyQueryCallbacks, are called if present.
    ///
    /// Has() has the same side effects as JavaScript's variable in object.
    /// For example, calling Has() on a revoked proxy will throw an exception.
    ///
    /// Note Has() converts the key to a name, which possibly calls back into
    /// JavaScript.
    ///
    /// See also Object::hasOwnProperty() and Object::HasRealNamedProperty()
    #[inline]
    pub fn has(&mut self, context: V8Context, key: V8Value) -> Maybe<bool> {
        unsafe {
            self.Has(context, key)
        }
    }

    #[inline]
    pub fn set_accessor(&mut self,
        context: V8Context,
        name: V8Name,
        getter: AccessorNameGetterCallback,
        setter: AccessorNameSetterCallback,
        data: MaybeLocal<Value>,
        settings: AccessControl,
        attribute: PropertyAttribute,
        getter_side_effect_type: SideEffectType,
        setter_side_effect_type: SideEffectType) -> Maybe<bool> {
        unsafe {
            self.SetAccessor(context,
                name,
                getter,
                setter,
                data,
                settings,
                attribute,
                getter_side_effect_type,
                setter_side_effect_type)
        }
    }

    #[inline]
    pub fn set_accessor_property(&mut self,
        name: V8Name,
        getter: V8Function,
        setter: V8Function,
        attribute: PropertyAttribute,
        settings: AccessControl) {
        unsafe {
            self.SetAccessorProperty(name,
                getter,
                setter,
                attribute,
                settings)
        }
    }
}

inherit!(Object, Value, Data);
inherit_local!(Object, Value, Data);

impl ValueTrait for Object {}
