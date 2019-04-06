## Rust Bindings To V8

Aims to become the best bridge between Rust & V8 ;)

NB: This crate assumes you have a pre-compiled v8 installed in your os, and with pkg-config support:

For example (/usr/local/share/pkgconfig/v8.pc):

```sh
prefix=/home/v8/source/v8

Name: v8
Description: v8
Version: 7.4.79
Cflags: -I${prefix}/include
Libs: -L${prefix}/lib -lv8_monolith
```


## Examples

### 1. Hello, World

```rust
let _platform = Platform::New();
let mut isolate = Isolate::New();

isolate.exec(move |context| {
    let source = V8String::New(r#"
        "Hello, World!"
    "#);
    let mut script = V8Script::New(context, source);
    let result: String = script.run(context).to_local_checked().into();
    println!("{}", result)
})
```

### 2. Wrapper Rust function into V8 FunctionTemplate

```rust
let text = "Hello from Rust!"
let mut function = FunctionT::Call(|args, rv| {
    println!("{}", text);
    rv.set::<V8Value>(args.at(0));
})
```

or set callback later:

```rust
let mut function = FunctionT::New();
function.set_call_closure(|args, rv| {
    rv.set::<V8Value>(args.at(0));
})
```

or use rust extern fn:

```rust
extern fn function_tpl(info: *const FunctionCallbackInfo) {
    unsafe {
        let args = *info;
        let mut rv = args.get_return_value();
        rv.set::<V8Value>(args.at(0));
    }
}
let data = V8Value::Empty();
let mut function = FunctionT::New();
function.set_call_handler(Some(function_tpl), Some(data));
```

## Contributing

## License
