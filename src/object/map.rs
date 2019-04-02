pub use crate::v8::raw::Map;

use crate::v8::{
    Object,
    Value,
    Data,
};

inherit!(Map, Object, Value, Data);
inherit_local!(Map, Object, Value, Data);
