use pkg_config::Library;
use std::env;
use std::path::PathBuf;

pub fn main() {
    let Library { include_paths, .. } = pkg_config::Config::new().probe("v8").unwrap();
    let cflags: Vec<String> = include_paths
        .clone()
        .into_iter()
        .map(|pathbuf| format!("-I{}", pathbuf.to_str().unwrap()))
        .collect();

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c++", "-std=c++14"])
        .rust_target(bindgen::RustTarget::Nightly)
        .opaque_type("std::.*")
        .blacklist_type("std::basic_string.*")
        .whitelist_type("v8::.*")
        .whitelist_function("v8::.*")
        .whitelist_var("v8::.*")
        .whitelist_type("std::unique_ptr\\<v8::Platform\\>")
        .whitelist_type("std::unique_ptr\\<v8::TracingController\\>")
        .enable_cxx_namespaces()
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .clang_args(cflags)
        .header("v8wrapper.h")
        .generate()
        .expect("Unable to generate v8.h bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("v8.rs"))
        .expect("Could not write bindings!");
}
