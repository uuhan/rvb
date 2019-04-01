use crate::v8::{
    Local,
    Value,
    raw::internal::Address,
};

extern "C" {
    fn V8_ReturnValue_SetNull(value: &ReturnValue);
    fn V8_ReturnValue_SetUndefined(value: &ReturnValue);
}

#[repr(C)]
pub struct ReturnValue<'a> {
    value_: &'a mut Address,
}

impl<'a> ReturnValue<'a> {
    pub fn get(&mut self) -> Local<Value> {
        unimplemented!()
    }

    pub fn set<T>(&mut self, value: <T as ReturnedValue>::Item)
        where T: ReturnedValue
        {
            T::set(value, self)
        }

    #[inline]
    pub fn set_null(&mut self) {
        unsafe {
            V8_ReturnValue_SetNull(self)
        }
    }

    #[inline]
    pub fn set_undefined(&mut self) {
        unsafe {
            V8_ReturnValue_SetUndefined(self)
        }
    }
}

pub trait ReturnedValue {
    type Item;

    fn set(value: Self::Item, rv: &ReturnValue);
}

impl ReturnedValue for bool {
    type Item = bool;
    fn set(value: Self::Item, rv: &ReturnValue) {
        extern "C" {
            fn V8_ReturnValue_SetBool(rv: &ReturnValue, value: bool);
        }

        unsafe {
            V8_ReturnValue_SetBool(rv, value)
        }
    }
}

impl ReturnedValue for f64 {
    type Item = f64;
    fn set(value: Self::Item, rv: &ReturnValue) {
        extern "C" {
            fn V8_ReturnValue_SetDouble(rv: &ReturnValue, value: f64);
        }

        unsafe {
            V8_ReturnValue_SetDouble(rv, value)
        }
    }
}

impl ReturnedValue for u32 {
    type Item = u32;
    fn set(value: Self::Item, rv: &ReturnValue) {
        extern "C" {
            fn V8_ReturnValue_SetUint32(rv: &ReturnValue, value: u32);
        }

        unsafe {
            V8_ReturnValue_SetUint32(rv, value)
        }
    }
}
impl ReturnedValue for i32 {
    type Item = i32;
    fn set(value: Self::Item, rv: &ReturnValue) {
        extern "C" {
            fn V8_ReturnValue_SetInt32(rv: &ReturnValue, value: i32);
        }

        unsafe {
            V8_ReturnValue_SetInt32(rv, value)
        }
    }
}

impl ReturnedValue for Local<Value> {
    type Item = Local<Value>;
    fn set(value: Self::Item, rv: &ReturnValue) {
        extern "C" {
            fn V8_ReturnValue_SetLocalValue(rv: &ReturnValue, value: Local<Value>);
        }

        unsafe {
            V8_ReturnValue_SetLocalValue(rv, value)
        }
    }
}

// TODO
// impl ReturnedValue for Persistent<Value> {}
// impl ReturnedValue for Global<Value> {}
// impl ReturnedValue for TracedGlobal<Value> {}
