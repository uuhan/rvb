#![allow(dead_code,non_snake_case)]
extern crate rvb as v8;
use v8::v8::{
    prelude::*,
    Platform,
    Isolate,
    ContextParams,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(move |_| {
        let mut global = ObjectT::New(None);
        let name = V8String::New("foo");
        let value = V8String::New("bar");
        global.set(name, value);
        let mut params = ContextParams::default();
        params.global_template = global.into();
        let ctx = V8Context::New(params);

        let source = V8String::New(r#"
                function concat(a, b) {
                  return a + b
                }
                concat("foo: ", foo)
            "#);
        let mut script = V8Script::New(ctx, source)?;
        let result: String = script.run(ctx)?.into();
        println!("{}", result);
        Ok(())
    }).unwrap();
}

struct HttpRequest;
impl HttpRequest {
    pub fn Path() -> String {
        unimplemented!()
    }
    pub fn Referer() -> String {
        unimplemented!()
    }
    pub fn Host() -> String{
        unimplemented!()
    }
    pub fn UserAgent() -> String{
        unimplemented!()
    }
}
