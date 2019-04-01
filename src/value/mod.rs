use crate::v8::{
    Value,
    Data,
};

mod string;
mod bigint;
mod boolean;
mod int32;
mod symbol;
mod uint32;

pub use string::*;
pub use bigint::*;
pub use boolean::*;
pub use int32::*;
pub use uint32::*;
pub use symbol::*;

inherit!(Value, Data);
inherit_local!(Value, Data);
