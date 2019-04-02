pub use crate::v8::raw::Int32;

use crate::v8::{
    Integer,
    Number,
    Primitive,
    Value,
    Data,
};

inherit!(Int32, Integer, Number, Primitive, Value, Data);
inherit_local!(Int32, Integer, Number, Primitive, Value, Data);
