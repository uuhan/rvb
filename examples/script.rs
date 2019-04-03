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
    Value,
    ObjectTemplate,
    FunctionTemplate,
    FunctionCallbackInfo,
};

extern fn print_fn (info: *const FunctionCallbackInfo) {
    unsafe {
        let this = (*info).this();
        let value: String = (*info).data().into();
        let mut rc = (*info).get_return_value();
        println!("Hello from Rust!");
        println!("args.Data(): {}", value);

        rc.set::<Local<Value>>(this.into());
    }
}

pub fn main() {
    let _platform = Platform::New();
    let script = fs::read_to_string("./script.js").expect("file script.js not found");
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let _scope = ContextScope::New(context);
        let mut global = Local::<ObjectTemplate>::New(None);
        let mut print = Local::<FunctionTemplate>::New(Some(print_fn));

        global.set(Local::<V8String>::New("global").into(), Local::<ObjectTemplate>::New(None).into());
        global.set(Local::<V8String>::New("setTimeout").into(), Local::<FunctionTemplate>::New(None).into());
        global.set(Local::<V8String>::New("print").into(), print.into());

        let mut params = ContextParams::default();
        params.global_template = global.into();
        let ctx = Local::<Context>::New(params);

        let _scope_2 = ContextScope::New(ctx);


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
