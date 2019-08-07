extern crate rvb;

use rvb::v8::{
    Platform,
    Isolate,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.add_message_listener(|_message| {
        println!("message listener");
    });
}
