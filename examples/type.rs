extern crate v8_rs as v8;
use v8::v8::prelude::*;
use v8::v8::{
    Platform,
    Isolate,
    Context,
    ContextScope,
    String as V8String,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let mut str1 = Local::<V8String>::New("foo");

        println!("str1 is name: {}", str1.is_name());
        println!("str1 is nubmer object: {}", str1.is_number_object());
    });
}
