use crate::v8::{
    raw,
    Name,
    Primitive,
    Value,
    Data,
};

pub use raw::Symbol;

inherit!(Symbol, Name, Primitive, Value, Data);
inherit_local!(Symbol, Name, Primitive, Value, Data);
