#![allow(warnings)]
extern crate v8_rs as v8;
use v8::v8::prelude::*;
use v8::v8::{
    Platform,
    Isolate,
    Context,
    ContextScope,
    Object,
    String as V8String,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();
    let mut undefined = isolate.undefined();

    isolate.exec(move |context| {
        let mut str1 = Local::<V8String>::New("foo");
        let mut obj1 = Local::<Object>::New();

        println!("str1 is name: {}", str1.is_name());
        println!("str1 is nubmer object: {}", str1.is_number_object());
        println!("obj1 is object: {}", obj1.is_object());
        println!("undefined is object: {}", undefined.is_object());
    });
}
