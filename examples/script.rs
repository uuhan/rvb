extern crate v8_rs as v8;
use std::fs;

use v8::v8::prelude::*;
use v8::v8::{
    Platform,
    Isolate,
    Context,
    ContextParams,
    ContextScope,
    String as V8String,
    Script,
    Local,
    ObjectTemplate,
};

pub fn main() {
    let _platform = Platform::New();
    let script = fs::read_to_string("./script.js").expect("file script.js not found");
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let _scope = ContextScope::New(context);
        let mut global = Local::<ObjectTemplate>::New();
        let mut window = Local::<ObjectTemplate>::New();


        let mut params = ContextParams::default();
        params.global_template = global.into();

        let ctx = Local::<Context>::New(params);
        let window_object = window.constructor(ctx).to_local_checked();
        global.set(Local::<V8String>::New("window").into(), window_object.into());
        let source = Local::<V8String>::New(script);
        let mut script = Local::<Script>::New(ctx, source);

        let result = script.run(ctx);

        if result.is_empty() {
            ()
        } else {
            let result: String = result.to_local_checked().into();
            println!("{}", result);
        }
    });
}
