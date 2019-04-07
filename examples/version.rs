extern crate v8_rs as v8;

use v8::utils::version;

pub fn main() {
    println!("V8 Version: {}", version());
}
