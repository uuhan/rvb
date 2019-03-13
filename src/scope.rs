use crate::raw;
use crate::Isolated;

pub struct HandleScope(raw::HandleScope);

impl Isolated for HandleScope {
    fn new() -> Self {
        let isolate = Self::isolate();
        unsafe {
            HandleScope(
                raw::HandleScope::new(isolate.0)
            )
        }
    }
}
