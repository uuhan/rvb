use crate::v8::{
    raw::Date,
    Object,
    Value,
    Data,
};

inherit!(Date, Object, Value, Data);
inherit_local!(Date, Object, Value, Data);
