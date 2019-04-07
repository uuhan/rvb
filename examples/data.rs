extern crate v8_rs;

use v8_rs::v8;
use v8::prelude::*;
use v8::{
    Platform,
    Isolate,
    ISOLATE_DATA_SLOT,
    IsolateData,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.set_data::<String>(1, "Come on, Isolate!".into());

    isolate.exec(move |_context| {
        let mut str1 = V8String::New("foo");

        println!("str1 is name: {}", str1.is_name());
        Ok(())
    });

    let v = isolate.get_data::<String>(1);
    let data = isolate.get_data::<IsolateData>(ISOLATE_DATA_SLOT);
    println!("Isolate Ref Count: {}", data.count);
    println!("{}", v);
}
