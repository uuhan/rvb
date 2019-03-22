pub use crate::v8::raw::Array;
pub use crate::v8::raw::ArrayBuffer;
pub use crate::v8::raw::ArrayBufferView;

use crate::v8::Isolated;

impl<'a> Isolated<'a> for Array {}
