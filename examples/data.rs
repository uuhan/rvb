extern crate v8_rs;

use v8_rs::v8;
use v8::prelude::*;
use v8::{
    Platform,
    Isolate,
    Context,
    ContextScope,
    ISOLATE_DATA_SLOT,
    IsolateData,
    String as V8String,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(move |context| {
        let mut str1 = Local::<V8String>::New("foo");

        println!("str1 is name: {}", str1.is_name());
    });

    let data = isolate.get_data::<IsolateData>(0);
    println!("{}", data.count);
}
