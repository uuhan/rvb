extern crate v8_rs;
use v8_rs::v8;
use v8::{
    Rooted,
    Isolated,
    Platform,
    Context,
    ContextScope,
    Isolate,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    ()
}

struct HttpRequest;
impl HttpRequest {
    pub fn Path() -> String {
        unimplemented!()
    }
    pub fn Referer() -> String {
        unimplemented!()
    }
    pub fn Host() -> String{
        unimplemented!()
    }
    pub fn UserAgent() -> String{
        unimplemented!()
    }
}
