extern crate rvb;

use rvb::v8::{
    Platform,
    Isolate,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.add_near_heap_limit_closure(|current, initial| {
        println!("{} / {}", current, initial);
        current
    });
}
