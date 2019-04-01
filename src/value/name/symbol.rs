use crate::v8::{
    raw,
    Name,
};

pub use raw::Symbol;

inherit!(Symbol, Name);
inherit_local!(Symbol, Name);
