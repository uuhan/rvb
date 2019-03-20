pub use crate::v8::raw::Boolean;

use crate::v8::{
    Value,
};

inherit!(Boolean, Value);
inherit_local!(Boolean, Value);
