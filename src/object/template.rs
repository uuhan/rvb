use std::mem;
use super::*;

use crate::function::*;
use crate::v8::prelude::*;
pub use crate::v8::raw::ObjectTemplate;

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

impl Local<ObjectTemplate> {
    /// Creates an ObjectTemplate.
    #[inline]
    pub fn New(constructor: Option<Local<FunctionTemplate>>) -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            match constructor {
                Some(constructor) =>
                    ObjectTemplate::New(isolate, constructor),
                None =>
                    ObjectTemplate::New(isolate, Local::<FunctionTemplate>::Empty()),
            }
        }
    }

    /// Get a template included in the snapshot by index.
    #[inline]
    pub fn FromSnapshot(index: usize) -> MaybeLocal<ObjectTemplate> {
        let isolate = Self::GetIsolate();
        unsafe {
            ObjectTemplate::FromSnapshot(isolate, index)
        }
    }

    /// Creates a new instance of this template.
    #[inline]
    pub fn new_instance(&mut self, ctx: Local<Context>) -> MaybeLocal<Object> {
        unsafe {
            self.NewInstance(ctx)
        }
    }


    /// Sets an accessor on the object template.
    ///
    /// Whenever the property with the given name is accessed on objects
    /// created from this ObjectTemplate the getter and setter callbacks
    /// are called instead of getting and setting the property directly
    /// on the JavaScript object.
    ///
    /// @param name The name of the property for which an accessor is added.
    /// @param getter The callback to invoke when getting the property.
    /// @param setter The callback to invoke when setting the property.
    /// @param data A piece of data that will be passed to the getter and setter
    ///   callbacks whenever they are invoked.
    /// @param settings Access control settings for the accessor. This is a bit
    ///   field consisting of one of more of
    ///   DEFAULT = 0, ALL_CAN_READ = 1, or ALL_CAN_WRITE = 2.
    ///   The default is to not allow cross-context access.
    ///   ALL_CAN_READ means that all cross-context reads are allowed.
    ///   ALL_CAN_WRITE means that all cross-context writes are allowed.
    ///   The combination ALL_CAN_READ | ALL_CAN_WRITE can be used to allow all
    ///   cross-context access.
    /// @param attribute The attributes of the property for which an accessor
    ///   is added.
    /// @param signature The signature describes valid receivers for the accessor
    ///   and is used to perform implicit instance checks against them. If the
    ///   receiver is incompatible (i.e. is not an instance of the constructor as
    ///   defined by FunctionTemplate::HasInstance()), an implicit TypeError is
    ///   thrown and no callback is invoked.
    #[inline]
    pub fn set_accessor(&mut self,
        name: Local<String>,
        getter: AccessorGetterCallback,
        setter: AccessorSetterCallback,
        data: Local<Value>,
        settings: AccessControl,
        attribute: PropertyAttribute,
        signature: Local<AccessorSignature>,
        getter_side_effect_type: SideEffectType,
        setter_side_effect_type: SideEffectType,
    )
    {
        unsafe {
            self.SetAccessor(name,
                getter,
                setter,
                data,
                settings,
                attribute,
                signature,
                getter_side_effect_type,
                setter_side_effect_type)
        }
    }

    /// Sets a named property handler on the object template.
    ///
    /// Whenever a property whose name is a string or a symbol is accessed on
    /// objects created from this object template, the provided callback is
    /// invoked instead of accessing the property directly on the JavaScript
    /// object.
    ///
    /// @param configuration The NamedPropertyHandlerConfiguration that defines the
    /// callbacks to invoke when accessing a property.
    #[inline]
    pub fn set_handler(&mut self, configuration: *const NamedPropertyHandlerConfiguration) {
        unsafe {
            self.SetHandler(configuration)
        }
    }

    /// Sets the callback to be used when calling instances created from
    /// this template as a function. If no callback is set, instances
    /// behave like normal JavaScript objects that cannot be called as a
    /// function.
    #[inline]
    pub fn set_call_as_function_handler(&mut self, callback: FunctionCallback, data: Local<Value>) {
        unsafe {
            self.SetCallAsFunctionHandler(callback, data)
        }
    }

    /// Sets the callback to be used with a Rust closure when calling
    /// instances created from this template as a function.
    #[inline]
    pub fn set_call_as_function_closure<F>(&mut self, callback: F)
        where F: FnMut(&FunctionCallbackInfo, &mut ReturnValue)
    {
        let callback: Box<Box<dyn FnMut(&FunctionCallbackInfo, &mut ReturnValue)>>
            = Box::new(Box::new(callback));
        let data = V8External::New(Box::into_raw(callback) as *mut std::ffi::c_void);
        unsafe {
            self.SetCallAsFunctionHandler(Some(function_template), data.into());
        }
    }

    /// Mark object instances of the template as undectable.
    ///
    /// In many ways, undetectable objects behave as though they are not
    /// there. They behave like 'undefined' in conditioals and when
    /// printed. However, properties can be accessed and called as on
    /// normal objects.
    #[inline]
    pub fn mark_as_undetectable(&mut self) {
        unsafe {
            self.MarkAsUndetectable()
        }
    }

    /// Gets the number of internal fields for objects generated from
    /// this template.
    #[inline]
    pub fn internal_field_count(&mut self) -> u32 {
        unsafe {
            self.InternalFieldCount() as u32
        }
    }

    /// Sets the number of internal fields for objects generated from
    /// this template.
    #[inline]
    pub fn set_internal_field_count(&mut self, value: u32) {
        unsafe {
            self.SetInternalFieldCount(value as ::std::os::raw::c_int)
        }
    }

    /// Returns true if the object will be an immutable prototype exotic object.
    #[inline]
    pub fn is_immutable_proto(&mut self) -> bool {
        unsafe {
            self.IsImmutableProto()
        }
    }

    /// Makes the ObjectTemplate for an immutable prototype exotic object, with an
    /// immutable __proto__.
    #[inline]
    pub fn set_immutable_proto(&mut self) {
        unsafe {
            self.SetImmutableProto()
        }
    }
}

inherit!(ObjectTemplate, Template, Data);
inherit_local!(ObjectTemplate, Template, Data);
impl TemplateTrait for ObjectTemplate {}

