extern crate rvb as v8;
use std::fs;

use v8::v8::prelude::*;
use v8::v8::{
    Platform,
    Isolate,
    ContextParams,
    ContextScope,
    V8Result,
};

pub fn main() -> V8Result<()> {
    let _platform = Platform::New();
    let script = fs::read_to_string("./script.js").expect("file script.js not found");
    let mut isolate = Isolate::New();

    let text = "Hello from Rust!";

    isolate.exec(move |context| {
        let _scope = ContextScope::New(context);
        let mut global = ObjectT::New(None);
        let print = FunctionT::Call(|args, rv| {
            let arg: String = args.at(0).into();
            println!("{}", arg);
            rv.set_undefined()
        });


        global.set(V8String::New("global"), ObjectT::New(None));
        global.set(V8String::New("print"), print);

        let mut params = ContextParams::default();
        params.global_template = global.into();
        let ctx = V8Context::New(params);

        let _scope_2 = ContextScope::New(ctx);


        let source = V8String::New(script);
        let mut script = V8Script::New(ctx, source)?;
        let result: String = script.run(ctx)?.into();
        println!("{}", result);

        Ok(())
    })
}
