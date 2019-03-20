pub use crate::v8::raw::Uint32;

use crate::v8::{
    Value,
};

inherit!(Uint32, Value);
inherit_local!(Uint32, Value);
