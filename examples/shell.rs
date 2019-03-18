#![allow(unused)]
extern crate v8_rs;
use v8_rs::v8::{
    self,
    Platform,
    Isolate,
    HandleScope,
    Rooted,
    Isolated,
    Local,
    Context,
    ContextScope,
    String as V8String,
    Script,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();
    unsafe {
        isolate.with(|context| {
            assert!(!context.is_empty());

            let scope = ContextScope::New(context);
        })
    }
    ()
}

fn create_shell_context(isolate: Isolate) {}
