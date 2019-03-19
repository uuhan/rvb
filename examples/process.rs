extern crate v8_rs;
use v8_rs::v8;
use v8::{
    Rooted,
    Local,
    Isolated,
    Platform,
    Context,
    ContextScope,
    Isolate,
    ObjectTemplate,
    FunctionTemplate,
    ContextParams,
    String as V8String,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.with(move |context| {
        let mut global = Local::<ObjectTemplate>::New();
        global.set(Local::<V8String>::New("foo"), Local::<V8String>::New("bar"));
        let mut params = ContextParams::default();
        params.global_template = global;
        let ctx = Local::<Context>::New(params);
    });
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
