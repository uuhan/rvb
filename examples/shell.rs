#![allow(unused)]
extern crate v8_rs;
use std::process;
use v8_rs::v8::{
    self,
    Platform,
    Isolate,
    HandleScope,
    Rooted,
    Isolated,
    Context,
    ContextScope,
    String as V8String,
    Script,
};

pub fn main() {
    let mut platform = Platform::New();
    let mut isolate = Isolate::New();
    isolate.exec(|context| {
        if context.is_empty() {
            eprintln!("Error create context!");
            process::exit(1);
        }
    })
}

fn create_shell_context(isolate: Isolate) {}
