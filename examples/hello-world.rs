#![allow(unused)]
extern crate v8_rs;
use v8_rs::v8::prelude::*;
use v8_rs::v8::{
    self,
    Platform,
    Isolate,
    HandleScope,
    Local,
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
        let mut script = Local::<Script>::New(context, source);
        let result: String = script.run(context).to_local_checked().into();
        println!("{}", result);
    });

    let mut isolate = Isolate::New();
    isolate.exec(move |context| {
        let scope = ContextScope::New(context);
        let source = V8String::New(r#"
                "Hello, " + "World!"
            "#);
        let mut script = Local::<Script>::New(context, source);
        let result: String = script.run(context).to_local_checked().into();
        println!("{}", result);
    });
}
