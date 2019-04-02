pub use crate::v8::raw::Uint32;

use crate::v8::{
    Integer,
    Number,
    Primitive,
    Value,
    Data,
};

inherit!(Uint32, Integer, Number, Primitive, Value, Data);
inherit_local!(Uint32, Integer, Number, Primitive, Value, Data);
