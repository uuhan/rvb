use v8_rs::{v8, Root, Isolated};
use std::ptr::null_mut;
use std::ffi::CStr;

extern {
    pub fn V8_ToLocalChecked(t: v8::raw::MaybeLocal<v8::raw::Value>) -> v8::raw::Local<v8::raw::Value>;
    pub fn V8_Context_Enter(c: v8::raw::Local<v8::raw::Context>);
    pub fn V8_Context_New(i: *mut v8::raw::Isolate) -> v8::raw::Local<v8::raw::Context>;
}

pub fn main() {
    unsafe {
        let platform = v8::Platform::new();
        let mut isolate = v8::Isolate::new();
        isolate.enter();

        let handle_scope = v8::HandleScope::new();

        let context = V8_Context_New(isolate.0);
        V8_Context_Enter(context);

        let source = v8::raw::String::NewFromUtf8(isolate.0, (*b"'Hello' + ', World!'\0").as_ptr() as *const i8, 0, -1);
        let mut script = v8::raw::Script::Compile(context, source, null_mut());
        let result = (*script.val_).Run(context);

        let t: *mut i8 = v8::raw::String_Utf8Value::new(isolate.0, V8_ToLocalChecked(result)).str_;

        let s = CStr::from_ptr(t).to_str().unwrap();

        println!("{}", s);

        isolate.exit();
    }
    println!("{}", "v8");
}
