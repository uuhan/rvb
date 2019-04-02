pub use crate::v8::raw::Set;

use crate::v8::{
    Object,
    Value,
    Data,
};

inherit!(Set, Object, Value, Data);
inherit_local!(Set, Object, Value, Data);
