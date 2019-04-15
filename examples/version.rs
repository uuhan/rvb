extern crate rvb as v8;

use v8::utils::version;

pub fn main() {
    println!("V8 Version: {}", version());
}
