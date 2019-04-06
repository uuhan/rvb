use crate::v8::{
    raw,
    Value,
    Data,
    ValueTrait,
};

mod bigint;
mod boolean;
mod int32;
mod uint32;
mod name;

pub use raw::Primitive;
pub use raw::Number;
pub use raw::Integer;

pub use bigint::*;
pub use boolean::*;
pub use int32::*;
pub use uint32::*;
pub use name::*;

inherit!(Value, Data);
inherit_local!(Value, Data);

inherit!(Primitive, Value, Data);
inherit_local!(Primitive, Value, Data);

inherit!(Number, Primitive, Value, Data);
inherit_local!(Number, Primitive, Value, Data);

inherit!(Integer, Number, Primitive, Value, Data);
inherit_local!(Integer, Number, Primitive, Value, Data);

impl ValueTrait for Primitive {}
