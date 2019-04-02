use crate::v8::{
    raw,
    Primitive,
    Value,
    Data,
};

pub use raw::Name;

mod string;
mod symbol;

pub use string::*;
pub use symbol::*;

inherit!(Name, Primitive, Value, Data);
