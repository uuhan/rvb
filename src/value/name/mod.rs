use crate::v8::{
    raw,
    Primitive,
};

pub use raw::Name;

mod string;
mod symbol;

pub use string::*;
pub use symbol::*;

inherit!(Name, Primitive);
