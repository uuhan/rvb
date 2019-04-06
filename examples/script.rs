#![allow(unused)]
#![feature(fnbox)]
extern crate v8_rs as v8;
use std::fs;
use std::boxed::FnBox;
use std::ffi::c_void;
use std::mem;
use std::ffi::CString;

use v8::v8::prelude::*;
use v8::v8::{
    Platform,
    Isolate,
    Context,
    ContextParams,
    ContextScope,
    Script,
    Local,
    Object,
    External,
    Value,
    Primitive,
    ObjectTemplate,
    FunctionTemplate,
    FunctionCallbackInfo,
};

pub fn main() {
    let _platform = Platform::New();
    let script = fs::read_to_string("./script.js").expect("file script.js not found");
    let mut isolate = Isolate::New();

    let text = "Hello from Rust!";

    isolate.exec(move |context| {
        let _scope = ContextScope::New(context);
        let mut global = ObjectT::New(None);
        let mut print = FunctionT::Call(|args, rv| {
            println!("{}", text);
            rv.set::<Local<Value>>(args.at(0));
        });


        global.set(V8String::New("global"), Local::<ObjectTemplate>::New(None));
        global.set(V8String::New("print"), print);

        let mut params = ContextParams::default();
        params.global_template = global.into();
        let ctx = Local::<Context>::New(params);

        let _scope_2 = ContextScope::New(ctx);


        let source = V8String::New(script);
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
