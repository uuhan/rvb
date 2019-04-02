pub use crate::v8::raw::BigInt;

use crate::v8::{
    Primitive,
    Value,
    Data,
};

inherit!(BigInt, Primitive, Value, Data);
inherit_local!(BigInt, Primitive, Value, Data);
