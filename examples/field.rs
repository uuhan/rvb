extern crate v8_rs as v8;
use v8::v8::{
    prelude::*,
    Platform,
    Isolate,
    Object,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(|ctx| {
        let mut object = V8Object::New();
        let count = object.internal_field_count();
        println!("{}", count);
        // object.set_internal_field(0, V8String::New("It's internal field."));
        // let value = object.get_internal_field::<V8Value>(1).unwrap();
        // println!("{}", Into::<String>::into(value));

        Ok(())
    }).unwrap();
}
