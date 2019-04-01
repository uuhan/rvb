pub use crate::v8::raw::BigInt;

use crate::v8::{
    Primitive,
};

inherit!(BigInt, Primitive);
inherit_local!(BigInt, Primitive);
