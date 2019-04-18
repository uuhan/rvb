pub use crate::v8::raw::HandleScope;
pub use crate::v8::raw::EscapableHandleScope;
pub use crate::v8::raw::Context_Scope as ContextScope;

use crate::v8::Isolated;
use crate::v8::Context;
use crate::v8::Local;
use crate::v8::raw::Isolate_GetCurrent;

impl<'a> Isolated<'a> for HandleScope {
    fn New() -> Box<Self> {
        let isolate = unsafe {Isolate_GetCurrent()};
        Box::new(
            unsafe {
                HandleScope::new(isolate)
            }
        )
    }
}

impl<'a> Isolated<'a> for EscapableHandleScope {
    fn New() -> Box<Self> {
        let isolate = Self::GetIsolate();
        Box::new(
            unsafe {
                EscapableHandleScope::new(isolate)
            }
        )
    }
}

impl ContextScope {
    pub fn New(mut context: Local<Context>) -> Self {
        unsafe {
            context.Enter();
        }
        ContextScope {
            context_: context,
        }
    }
}

/// Enter a Context
/// v8::Context::Scope context_scope(context)
impl<'a> Isolated<'a> for ContextScope {}

/// Exit a Context
impl Drop for ContextScope {
    fn drop(&mut self) {
        unsafe {
            self.context_.Exit()
        }
    }
}
