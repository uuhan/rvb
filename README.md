## Rust Bindings To V8

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
    let source = Local::<V8String>::New(r#"
        "Hello, World!"
    "#);
    let mut script = Local::<Script>::New(context, source);
    let result: String = script.run(context).to_local_checked().into();
    println!("{}", result)
})
```

## Contributing

## License
