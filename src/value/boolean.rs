pub use crate::v8::raw::Boolean;

use crate::v8::{
    Primitive,
    Value,
    Data,
};

inherit!(Boolean, Primitive, Value, Data);
inherit_local!(Boolean, Primitive, Value, Data);
