use std::ffi::CStr;
use std::convert::Into;

use crate::v8::{
    raw,
    Local,
    Isolated,
};

pub use crate::v8::raw::Value;

/// isomorphism to v8::Value base class
pub trait ValueTrait {
    #[inline]
    base!(is_true, bool, Value, IsTrue);
    #[inline]
    base!(is_false, bool, Value, IsFalse);
    #[inline]
    base!(is_name, bool, Value, IsName);
    #[inline]
    base!(is_symbol, bool, Value, IsSymbol);
    #[inline]
    base!(is_function, bool, Value, IsFunction);
    #[inline]
    base!(is_array, bool, Value, IsArray);
    #[inline]
    base!(is_object, bool, Value, IsObject);
    #[inline]
    base!(is_bigint, bool, Value, IsBigInt);
    #[inline]
    base!(is_boolean, bool, Value, IsBoolean);
    #[inline]
    base!(is_number, bool, Value, IsNumber);
    #[inline]
    base!(is_external, bool, Value, IsExternal);
    #[inline]
    base!(is_int32, bool, Value, IsInt32);
    #[inline]
    base!(is_uint32, bool, Value, IsUint32);
    #[inline]
    base!(is_date, bool, Value, IsDate);
    #[inline]
    base!(is_argumets_object, bool, Value, IsArgumentsObject);
    #[inline]
    base!(is_bigint_object, bool, Value, IsBigIntObject);
    #[inline]
    base!(is_boolean_object, bool, Value, IsBooleanObject);
    #[inline]
    base!(is_number_object, bool, Value, IsNumberObject);
    #[inline]
    base!(is_string_object, bool, Value, IsStringObject);
    #[inline]
    base!(is_symbol_object, bool, Value, IsSymbolObject);
    #[inline]
    base!(is_native_error, bool, Value, IsNativeError);
    #[inline]
    base!(is_regexp, bool, Value, IsRegExp);
    #[inline]
    base!(is_async_function, bool, Value, IsAsyncFunction);
    #[inline]
    base!(is_generator_function, bool, Value, IsGeneratorFunction);
    #[inline]
    base!(is_promise, bool, Value, IsPromise);
    #[inline]
    base!(is_map, bool, Value, IsMap);
    #[inline]
    base!(is_set, bool, Value, IsSet);
    #[inline]
    base!(is_map_iterator, bool, Value, IsMapIterator);
    #[inline]
    base!(is_set_iterator, bool, Value, IsSetIterator);
    #[inline]
    base!(is_weak_map, bool, Value, IsWeakMap);
    #[inline]
    base!(is_weak_set, bool, Value, IsWeakSet);
    #[inline]
    base!(is_array_buffer, bool, Value, IsArrayBuffer);
    #[inline]
    base!(is_array_bufferview, bool, Value, IsArrayBufferView);
    #[inline]
    base!(is_typed_array, bool, Value, IsTypedArray);
    #[inline]
    base!(is_uint8_array, bool, Value, IsUint8Array);
    #[inline]
    base!(is_uint8_clamped_array, bool, Value, IsUint8ClampedArray);
    #[inline]
    base!(is_int8_array, bool, Value, IsInt8Array);
    #[inline]
    base!(is_uint16_array, bool, Value, IsUint16Array);
    #[inline]
    base!(is_int16_array, bool, Value, IsInt16Array);
    #[inline]
    base!(is_uint32_array, bool, Value, IsUint32Array);
    #[inline]
    base!(is_int32_array, bool, Value, IsInt32Array);
    #[inline]
    base!(is_float32_array, bool, Value, IsFloat32Array);
    #[inline]
    base!(is_float64_array, bool, Value, IsFloat64Array);
    #[inline]
    base!(is_bigint64_array, bool, Value, IsBigInt64Array);
    #[inline]
    base!(is_biguint64_array, bool, Value, IsBigUint64Array);
    #[inline]
    base!(is_dataview, bool, Value, IsDataView);
    #[inline]
    base!(is_shared_array_buffer, bool, Value, IsSharedArrayBuffer);
    #[inline]
    base!(is_proxy, bool, Value, IsProxy);
    #[inline]
    base!(is_webassembly_compiled_module, bool, Value, IsWebAssemblyCompiledModule);
    #[inline]
    base!(is_module_namespace_object, bool, Value, IsModuleNamespaceObject);
    // TODO: convert function
}

impl ValueTrait for Value {}

/// cast local value into string
/// use v8::String::Utf8Value
impl Into<String> for Local<Value> {
    fn into(self) -> String {
        let isolate = Self::GetIsolate();
        unsafe {
            let ps = raw::String_Utf8Value::new(isolate, self).str_;
            CStr::from_ptr(ps).to_owned().into_string().unwrap_or(format!("{:?}", self))
        }
    }
}
