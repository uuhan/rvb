use std::ptr;
use crate::v8::{MaybeLocal, Local, Context, raw::Value, String, raw};

pub use raw::Script;
pub use raw::UnboundScript;

impl Local<Script> {
    /// A shorthand for ScriptComiler::Compile().
    #[inline]
    pub fn New(context: Local<Context>, source: Local<String>) -> Self {
        unsafe {
            raw::Script::Compile(context, source, ptr::null_mut()).to_local_checked()
        }
    }

    /// Runs the script returning the resulting value. It will be run in the
    /// context in which it was created (ScriptCompiler::CompilBound or
    /// UnboundScript::BindToCurrentContext()).
    #[inline]
    pub fn run (&mut self, context: Local<Context>) -> MaybeLocal<Value> {
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
