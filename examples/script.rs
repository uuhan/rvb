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
    V8String,
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

extern fn print_fn (info: *const FunctionCallbackInfo) {
    unsafe {
        let args = &*info;
        let value: Local<External> = Local::<External>::from(args.data());
        let mut rc = args.get_return_value();
        println!("Hello from Rust!");

        rc.set::<Local<Value>>(args.at(0));
    }
}

pub fn main() {
    let _platform = Platform::New();
    let script = fs::read_to_string("./script.js").expect("file script.js not found");
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let _scope = ContextScope::New(context);
        let mut global = Local::<ObjectTemplate>::New(None);
        let mut print = Local::<FunctionTemplate>::New(None);
        let f: Box<FnBox()> = Box::new(|| {
            println!("from closure");
        });

        // print.set_call_handler(Some(print_fn), None);
        print.set_call_closure(|args, rv| {
            println!("Call From Closure!");
            rv.set::<Local<Value>>(args.this().into());
        });

        global.set(Local::<V8String>::New("global"), Local::<ObjectTemplate>::New(None));
        global.set(Local::<V8String>::New("setTimeout"), Local::<FunctionTemplate>::New(None));
        global.set(Local::<V8String>::New("print"), print);

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
