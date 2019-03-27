extern crate v8_rs;

use v8_rs::v8;
use v8::prelude::*;
use v8::{
    Platform,
    Isolate,
    Context,
    ContextScope,
    String as V8String,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    let s = "abcdef".to_string();
    isolate.set_data(0, s);

    isolate.exec(move |context| {
        let mut str1 = Local::<V8String>::New("foo");

        println!("str1 is name: {}", str1.is_name());
    });

    let data = isolate.get_data::<String>(0);
    println!("{}", *data);
    *data = "Hello, world!".into();
    let data = isolate.get_data::<String>(0);
    println!("{}", *data);
}
