## Rust Bindings To V8

NB: This crate assume you have a pre-compiled v8 installed in your os, and with pkg-config support:

For example (/usr/local/share/pkgconfig/v8.pc):

```sh
prefix=/usr/local/Cellar/v8/7.2.502.25

Name: v8
Description: v8
Version: 7.2.502.25
Cflags: -I${prefix}/include
Libs: -L${prefix}/lib -lv8 -lv8_libbase -lv8_libplatform
```

