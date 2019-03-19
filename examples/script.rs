extern crate v8_rs as v8;

use v8::v8::{Context, ContextScope, Isolate, Isolated, Platform, Rooted};

pub fn main() {
    let _platform = Platform::New();
}
