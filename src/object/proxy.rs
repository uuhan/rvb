pub use crate::v8::raw::Proxy;
use crate::v8::{
    Object,
    Value,
    Data,
};

inherit!(Proxy, Object, Value, Data);
inherit_local!(Proxy, Object, Value, Data);
