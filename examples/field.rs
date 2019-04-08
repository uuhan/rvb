extern crate v8_rs as v8;
use std::mem;

use v8::v8::{
    prelude::*,
    Platform,
    Isolate,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.exec(|ctx| {
        let mut objt = ObjectT::New(None);
        objt.set_internal_field_count(1);
        let mut object = objt.new_instance(ctx).to_local_checked()?;
        let count = object.internal_field_count();

        let callback: Box<Box<FnMut()>> = Box::new(Box::new(|| {
            println!("I am from internal field.");
        }));

        object.set_internal_field(0, V8External::New(Box::into_raw(callback) as *mut std::ffi::c_void));

        let closure_ptr = object.get_internal_field::<V8External>(0).value();
        let closure: &mut Box<FnMut()> = unsafe { mem::transmute(closure_ptr) };

        println!("object has {} internal field.", count);
        closure();

        Ok(())
    }).unwrap();
}
