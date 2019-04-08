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
        let mut objt = ObjectT::New(None);
        objt.set_internal_field_count(1);
        let mut object = objt.new_instance(ctx).to_local_checked()?;
        let count = object.internal_field_count();
        object.set_internal_field(0, V8String::New("It's internal field."));
        let value = object.get_internal_field::<V8Value>(0);
        println!("object has {} internal field.", count);
        println!("{}", Into::<String>::into(value));

        Ok(())
    }).unwrap();
}
