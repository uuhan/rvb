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
    String as V8String,
    Script,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let scope = ContextScope::New(context);
        let source = Local::<V8String>::New(r#"
                function concat(a, b) {
                  return a + b
                }
                concat("1", 1)
            "#);
        let mut script = Local::<Script>::New(context, source);
        let result: String = script.run(context).to_local_checked().into();
        println!("{}", result);
    });

    isolate.exec(move |context| {
        let scope = ContextScope::New(context);
        let source = Local::<V8String>::New(r#"
                "Hello, " + "World!"
            "#);
        let mut script = Local::<Script>::New(context, source);
        let result: String = script.run(context).to_local_checked().into();
        println!("{}", result);
    });
}
