#![allow(unused)]
extern crate rvb;
use rvb::v8::prelude::*;
use rvb::v8::{
    self,
    Platform,
    Isolate,
    HandleScope,
    Context,
    ContextScope,
    Script,
};

pub fn main() {
    let platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let scope = ContextScope::New(context);
        let source = V8String::New(r#"
                "Hello, " + "World!"
            "#);
        let mut script = V8Script::New(context, source)?;
        let result: String = script.run(context)?.into();
        println!("{}", result);
        Ok(())
    });
}
