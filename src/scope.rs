pub use crate::v8::raw::HandleScope;
use crate::v8::Isolated;

impl Isolated for HandleScope {
    fn New() -> Box<Self> {
        let isolate = Self::GetIsolate();
        Box::new(
            unsafe {
                HandleScope::new(isolate.0)
            }
        )
    }
}
