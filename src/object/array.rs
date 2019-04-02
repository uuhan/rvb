pub use crate::v8::raw::Array;
pub use crate::v8::raw::ArrayBuffer;
pub use crate::v8::raw::ArrayBufferView;

use crate::v8::{
    Isolated,
    Object,
    Value,
    Data,
};

impl<'a> Isolated<'a> for Array {}

inherit!(Array, Object, Value, Data);
inherit_local!(Array, Object, Value, Data);
