pub use crate::v8::raw::RegExp;
use crate::v8::{
    Object,
    Value,
    Data,
};

inherit!(RegExp, Object, Value, Data);
inherit_local!(RegExp, Object, Value, Data);
