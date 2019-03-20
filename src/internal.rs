use std::ops::Deref;
use std::ops::DerefMut;
use std::marker::PhantomData;
use std::ptr;
use std::default::Default;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;
use std::convert::Into;

use crate::v8::raw;
pub use crate::v8::raw::{
    Local,
    MaybeLocal,
    DeserializeInternalFieldsCallback,
    TryCatch,
    Name,
    Data,
    Template,

    PropertyAttribute,
    PropertyAttribute_None,
    PropertyAttribute_ReadOnly,
    PropertyAttribute_DontEnum,
    PropertyAttribute_DontDelete,
};

use crate::v8::Value;
use crate::v8::Isolate;

extern "C" {
    pub fn V8_To_Local_Checked(value: MaybeLocal<*mut c_void>) -> Local<*mut c_void>;
}

pub struct Address(*mut raw::internal::Address);
pub trait PersistentValue<T> {}

/// isomorphism to v8::Template base class
pub trait V8Template {
    /// NB: use std::mem::transmute_copy to reinterpert_cast class to it's base class
    /// should only impl this trait for ObjectTemplate & FunctionTemplate...
    fn set(&mut self, name: Local<Name>, value: Local<Data>) {
        unsafe {
            let base = mem::transmute_copy::<&mut Self, &mut Template>(&self);
            base.Set(
                name,
                value,
                PropertyAttribute_None)
        }
    }
}

/// isomorphism to v8::Value base class
pub trait V8Value {
    #[inline]
    base!(is_true, bool, Value, IsTrue);
    base!(is_false, bool, Value, IsFalse);
    base!(is_name, bool, Value, IsName);
    base!(is_symbol, bool, Value, IsSymbol);
    base!(is_function, bool, Value, IsFunction);
    base!(is_array, bool, Value, IsArray);
    base!(is_object, bool, Value, IsObject);
    base!(is_bigint, bool, Value, IsBigInt);
    base!(is_boolean, bool, Value, IsBoolean);
    base!(is_number, bool, Value, IsNumber);
    base!(is_external, bool, Value, IsExternal);
    base!(is_int32, bool, Value, IsInt32);
    base!(is_uint32, bool, Value, IsUint32);
    base!(is_date, bool, Value, IsDate);
    base!(is_argumets_object, bool, Value, IsArgumentsObject);
    base!(is_bigint_object, bool, Value, IsBigIntObject);
    base!(is_boolean_object, bool, Value, IsBooleanObject);
    base!(is_number_object, bool, Value, IsNumberObject);
    base!(is_string_object, bool, Value, IsStringObject);
    base!(is_symbol_object, bool, Value, IsSymbolObject);
    base!(is_native_error, bool, Value, IsNativeError);
    base!(is_regexp, bool, Value, IsRegExp);
    base!(is_async_function, bool, Value, IsAsyncFunction);
    base!(is_generator_function, bool, Value, IsGeneratorFunction);
    base!(is_promise, bool, Value, IsPromise);
    base!(is_map, bool, Value, IsMap);
    base!(is_set, bool, Value, IsSet);
    base!(is_map_iterator, bool, Value, IsMapIterator);
    base!(is_set_iterator, bool, Value, IsSetIterator);
    base!(is_weak_map, bool, Value, IsWeakMap);
    base!(is_weak_set, bool, Value, IsWeakSet);
    base!(is_array_buffer, bool, Value, IsArrayBuffer);
    base!(is_array_bufferview, bool, Value, IsArrayBufferView);
    base!(is_typed_array, bool, Value, IsTypedArray);
    base!(is_uint8_array, bool, Value, IsUint8Array);
    base!(is_uint8_clamped_array, bool, Value, IsUint8ClampedArray);
    base!(is_int8_array, bool, Value, IsInt8Array);
    base!(is_uint16_array, bool, Value, IsUint16Array);
    base!(is_int16_array, bool, Value, IsInt16Array);
    base!(is_uint32_array, bool, Value, IsUint32Array);
    base!(is_int32_array, bool, Value, IsInt32Array);
    base!(is_float32_array, bool, Value, IsFloat32Array);
    base!(is_float64_array, bool, Value, IsFloat64Array);
    base!(is_bigint64_array, bool, Value, IsBigInt64Array);
    base!(is_biguint64_array, bool, Value, IsBigUint64Array);
    base!(is_dataview, bool, Value, IsDataView);
    base!(is_shared_array_buffer, bool, Value, IsSharedArrayBuffer);
    base!(is_proxy, bool, Value, IsProxy);
    base!(is_webassembly_compiled_module, bool, Value, IsWebAssemblyCompiledModule);
    base!(is_module_namespace_object, bool, Value, IsModuleNamespaceObject);
    // TODO: convert function
}

/// an object can be enter in or exit out
pub trait Rooted {
    fn allocate() -> Self;
    fn enter(&mut self);
    fn exit(&mut self);
}

/// Object Should Live In an Isolate instance
pub trait Isolated {
    fn New() -> Box<Self> {
        unsafe {
            // TODO: seems not good
            mem::uninitialized()
        }
    }
    fn GetIsolate() -> Isolate {
        unsafe {
            let isolate = raw::Isolate::GetCurrent();
            assert!(!isolate.is_null());
            Isolate(isolate)
        }
    }
}

/// isomorphism to v8::Local<T>
impl<T> Local<T> {
    /// empty Local<T>
    /// v8::Local<T>()
    pub fn Empty() -> Self {
        Local {
            val_: ptr::null_mut(),
            _phantom_0: PhantomData,
        }
    }

    /// if Local<T> is empty
    pub fn is_empty(self) -> bool {
        self.val_.is_null()
    }
}

/// local value auto deref
impl<T> Deref for Local<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.val_
        }
    }
}

/// local value auto deref
impl<T> DerefMut for Local<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.val_
        }
    }
}

/// local value lives in an isolate instance
impl<T> Isolated for Local<T> {}

/// TryCatch Isolated
impl Isolated for TryCatch {
    fn New() -> Box<Self> {
        let isolate = Self::GetIsolate();
        Box::new(
            unsafe {
                TryCatch::new(isolate.0)
            }
        )
    }
}

/// cast local value into string
/// use v8::String::Utf8Value
impl Into<String> for Local<Value> {
    fn into(self) -> String {
        let isolate = Local::<Value>::GetIsolate();
        unsafe {
            let ps = raw::String_Utf8Value::new(isolate.0, self).str_;
            CStr::from_ptr(ps).to_owned().into_string().unwrap_or(format!("{:?}", self))
        }
    }
}

/// maybe local to local value
impl<T> MaybeLocal<T> {
    /// empty MaybeLocal<T>
    /// v8::MaybeLocal<T>()
    pub fn Empty() -> Self {
        MaybeLocal {
            val_: ptr::null_mut(),
            _phantom_0: PhantomData,
        }
    }

    /// if this maybelocal value is nothing
    pub fn is_empty(&self) -> bool {
        self.val_.is_null()
    }

    /// unwrap maybelocal to local
    pub fn to_local_checked(self) -> Local<T> {
        unsafe {
            mem::transmute(
                V8_To_Local_Checked(mem::transmute(self))
            )
        }
    }
}

/// convert Local<T> to MaybeLocal<T>
impl<T> Into<MaybeLocal<T>> for Local<T> {
    fn into(self) -> MaybeLocal<T> {
        let Local {
            val_, _phantom_0,
        } = self;

        MaybeLocal {
            val_,
            _phantom_0,
        }
    }
}

/// Default DeserializeInternalFieldsCallback
impl Default for DeserializeInternalFieldsCallback {
    fn default() -> Self {
        DeserializeInternalFieldsCallback {
            callback: None,
            data: ptr::null_mut(),
        }
    }
}
