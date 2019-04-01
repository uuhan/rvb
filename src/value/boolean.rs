pub use crate::v8::raw::Boolean;

use crate::v8::{
    Primitive,
};

inherit!(Boolean, Primitive);
inherit_local!(Boolean, Primitive);
