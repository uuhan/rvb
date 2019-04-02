use crate::v8::{
    raw::Promise,
    Object,
    Value,
    Data,
};


inherit!(Promise, Object, Value, Data);
inherit_local!(Promise, Object, Value, Data);
