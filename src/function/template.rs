use super::*;
pub use crate::v8::raw::FunctionTemplate;

impl<'a> Isolated<'a> for FunctionTemplate {}
impl TemplateTrait for FunctionTemplate {}

#[inline]
extern fn function_template(info: *const FunctionCallbackInfo) {
    unsafe {
        let args = &*info;
        let external = V8External::from(args.data());
        let external_ptr = external.value();
        let ref mut rv = args.get_return_value();

        let closure: &mut Box<FnMut(*const FunctionCallbackInfo, &mut ReturnValue)>
            = mem::transmute(external_ptr);
        closure(args, rv);
    }
}

impl FunctionT {
    /// Create a function template.
    #[inline]
    pub fn New() -> Self {
        let isolate = Self::GetIsolate();
        unsafe {
            FunctionTemplate::New(
                isolate,
                None,
                V8Value::Empty(),
                V8Signature::Empty(),
                0,
                ConstructorBehavior_kAllow,
                SideEffectType_kHasSideEffect,
                )
        }
    }

    /// Create a function template.
    #[inline]
    pub fn Call<F>(callback: F) -> Self
        where F: FnMut(&FunctionCallbackInfo, &mut ReturnValue)
        {
            let isolate = Self::GetIsolate();
            let callback: Box<Box<FnMut(&FunctionCallbackInfo, &mut ReturnValue)>>
                = Box::new(Box::new(callback));
            let data = V8External::New(Box::into_raw(callback) as *mut c_void);
            unsafe {
                FunctionTemplate::New(
                    isolate,
                    Some(function_template),
                    data.into(),
                    V8Signature::Empty(),
                    0,
                    ConstructorBehavior_kAllow,
                    SideEffectType_kHasSideEffect,
                    )
            }
        }

    /// Set the call-handler callback for a FunctionTemplate.
    /// This callback is called whenever the function created from this
    /// FunctionTemplate is called.
    #[inline]
    pub fn set_call_handler(&mut self, handler: FunctionCallback, data: Option<V8Value>) -> &mut Self {
        unsafe {
            match data {
                Some(data) => self.SetCallHandler(handler, data.into(), SideEffectType_kHasSideEffect),
                None => self.SetCallHandler(handler, V8Value::Empty(), SideEffectType_kHasSideEffect),
            }
        }
        self
    }

    /// Set the callback for a FunctionTemplate by closure.
    #[inline]
    pub fn set_call_closure<F>(&mut self, callback: F)
        where F: FnMut(&FunctionCallbackInfo, &mut ReturnValue)
        {
            let callback: Box<Box<FnMut(&FunctionCallbackInfo, &mut ReturnValue)>>
                = Box::new(Box::new(callback));
            let data = V8External::New(Box::into_raw(callback) as *mut c_void);
            self.set_call_handler(Some(function_template), Some(data.into()));
        }

    /// Returns the unique function instance in the current execution context.
    #[inline]
    pub fn get_function(&mut self, context: V8Context) -> MaybeLocal<Function> {
        unsafe {
            self.GetFunction(context)
        }
    }

    /// Set the predefined length property for the FunctionTemplate.
    #[inline]
    pub fn set_length(&mut self, length: i32) {
        unsafe {
            self.SetLength(length.into())
        }
    }

    /// Set the class name of the FunctionTemplate. This is used for
    /// printing objects created with the function created from the
    /// FunctionTemplate as its constructor.
    #[inline]
    pub fn set_class_name(&mut self, name: V8String) {
        unsafe {
            self.SetClassName(name)
        }
    }

    /// Causes the function template to inherit from a parent function template.
    /// This means the function's prototype.__proto__ is set to the parent
    /// function's prototype.
    #[inline]
    pub fn inherit(&mut self, parent: Self) {
        unsafe {
            self.Inherit(parent)
        }
    }

    /// Get the InstanceTemplate.
    #[inline]
    pub fn instance_template(&mut self) -> ObjectT {
        unsafe {
            self.InstanceTemplate()
        }
    }

    /// A PrototypeTemplate is the template used to create the prototype object
    /// of the function created by this template.
    #[inline]
    pub fn prototype_template(&mut self) -> ObjectT {
        unsafe {
            self.PrototypeTemplate()
        }
    }

    /// A PrototypeProviderTemplate is another function template whose prototype
    /// property is used for this template. This is manually exclusive with setting
    /// a prototype template indirectly by calling PrototypeTemplate() or using
    /// Inherit().
    #[inline]
    pub fn set_property_provider_template(&mut self, prototype_provider: FunctionT) {
        unsafe {
            self.SetPrototypeProviderTemplate(prototype_provider)
        }
    }

    /// When set to true, no access check will be performed on the receiver of a
    /// function call. Currently defaults to true, but this is subject to change.
    #[inline]
    pub fn set_accept_any_receiver(&mut self, value: bool) {
        unsafe {
            self.SetAcceptAnyReceiver(value)
        }
    }
}

inherit!(FunctionTemplate, Template, Data);
inherit_local!(FunctionTemplate, Template, Data);
impl ValueTrait for FunctionTemplate {}
