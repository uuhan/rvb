extern crate rvb as v8;
use v8::v8::{
    Platform,
    Isolate,
};

fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.with_locker(|mut ctx| {
        let _ = ctx.global();
        println!("run within locker");
    });
}
