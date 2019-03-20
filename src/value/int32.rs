pub use crate::v8::raw::Int32;

use crate::v8::{
    Value,
};

inherit!(Int32, Value);
inherit_local!(Int32, Value);
