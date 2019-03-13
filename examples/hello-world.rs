use v8_rs::v8::{
    self,
    Rooted,
    Isolated,
    Local,
    Context,
    String as V8String,
    Script,
};

pub fn main() {
    unsafe {
        let platform = v8::Platform::new();
        let mut isolate = v8::Isolate::new();
        isolate.enter();

        let handle_scope = v8::HandleScope::new(isolate.0);

        let mut context = Local::<Context>::New();
        context.enter();

        let source = Local::<V8String>::New("'Hello' + ', World!'");
        let mut script = Local::<Script>::New(context, source);
        let result: String = script.Run(context).to_local_checked().into();
        println!("{}", result);

        isolate.exit();
    }
}
