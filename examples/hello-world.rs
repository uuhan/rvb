use v8_ffi::v8;

pub fn main() {
    unsafe {
        let platform = v8::platform::NewDefaultPlatform(5, 0, 0, 0);
        v8::V8_ShutdownPlatform();
    }
    println!("{}", "v8")
}
