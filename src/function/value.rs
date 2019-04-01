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
pub struct ReturnValue {
    value_: *mut Address,
}

impl ReturnValue {
    pub fn get(&self) -> Local<Value> {
        unimplemented!()
    }

    pub fn set<T>(&self, value: <T as ReturnedValue>::Item)
        where T: ReturnedValue
        {
            T::set(value, self)
        }

    #[inline]
    pub fn set_null(&self) {
        unsafe {
            V8_ReturnValue_SetNull(self)
        }
    }

    #[inline]
    pub fn set_undefined(&self) {
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

// impl ReturnedValue for f64 {}
// impl ReturnedValue for u32 {}
// impl ReturnedValue for i32 {}

// impl ReturnedValue for Local<Value> {}

// TODO
// impl ReturnedValue for Persistent<Value> {}
// impl ReturnedValue for Global<Value> {}
// impl ReturnedValue for TracedGlobal<Value> {}
