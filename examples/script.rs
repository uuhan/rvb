extern crate v8_rs as v8;
use std::fs;

use v8::v8::{Context, ContextScope, Isolate, Isolated, Platform, Rooted, String as V8String, Script, Local};

pub fn main() {
    let _platform = Platform::New();
    let script = fs::read_to_string("./script.js").expect("file script.js not found");
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let scope = ContextScope::New(context);
        let source = Local::<V8String>::New(script);
        let mut script = Local::<Script>::New(context, source);

        let result = script.run(context);

        if result.is_empty() {
            println!("{:?}", result);
        } else {
            let result: String = result.to_local_checked().into();
            println!("{}", result);
        }
    });
}
