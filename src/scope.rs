pub use crate::v8::raw::HandleScope;
use crate::v8::Isolated;

impl Isolated for HandleScope {}
