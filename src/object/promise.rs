use crate::v8::{
    Object,
    Value,
    Data,
};

pub use crate::v8::raw::{
    Promise,
    PromiseHook,
    PromiseHookType,
    PromiseHookType_kInit,
    PromiseHookType_kResolve,
    PromiseHookType_kBefore,
    PromiseHookType_kAfter,
};

inherit!(Promise, Object, Value, Data);
inherit_local!(Promise, Object, Value, Data);
