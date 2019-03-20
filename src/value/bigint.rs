pub use crate::v8::raw::BigInt;

use crate::v8::{
    Value,
};

inherit!(BigInt, Value);
inherit_local!(BigInt, Value);
