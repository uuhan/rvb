## Rust Bindings To V8

NB: This crate assume you have a pre-compiled v8 installed in your os, and with pkg-config support:

For example (/usr/local/share/pkgconfig/v8.pc):

```sh
prefix=/home/v8/source/v8

Name: v8
Description: v8
Version: 7.4.79
Cflags: -I${prefix}/include
Libs: -L${prefix}/lib -lv8_monolith
```

