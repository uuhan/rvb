use std::ptr;
use crate::v8::{
    prelude::*,
    raw,
    V8Result,
    V8Error,
    MaybeLocal,
    Local,
    Value,
};

pub use raw::Script;
pub use raw::UnboundScript;

impl Local<Script> {
    /// A shorthand for ScriptComiler::Compile().
    #[inline]
    pub fn New(context: V8Context, source: V8String) -> V8Result<Self> {
        unsafe {
            match raw::Script::Compile(context, source, ptr::null_mut()).to_local_checked() {
                Ok(value) => Ok(value),
                Err(_) => Err(V8Error::V8ScriptCompileErr),
            }
        }
    }

    /// Runs the script returning the resulting value. It will be run in the
    /// context in which it was created (ScriptCompiler::CompilBound or
    /// UnboundScript::BindToCurrentContext()).
    #[inline]
    pub fn run (&mut self, context: V8Context) -> MaybeLocal<Value> {
        unsafe {
            self.Run(context)
        }
    }

    /// Returns the corresponding context-unbound script.
    #[inline]
    pub fn get_unbound_script(&mut self) -> Local<UnboundScript> {
        unsafe {
            self.GetUnboundScript()
        }
    }
}
