#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::black_box;
use std::mem;
use v8_rs::v8::{
    prelude::*,
    Platform,
    Isolate,
};

fn fibonacci(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut c = 0u64;

    if n == 0 {
        return 0
    }

    for _ in 0..(n+1) {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}

fn fib_bench(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn boxed_fn() {
    let closure = || {};
    let closure: Box<Box<FnMut()>>
        = Box::new(Box::new(closure));

    let closure: &mut Box<FnMut()> = unsafe { mem::transmute(closure) };
    closure()
}

fn boxed_fn_bench(c: &mut Criterion) {
    c.bench_function(
        "boxed closure",
        |b| b.iter(|| boxed_fn())
    );
}

criterion_group!(
    benches,
    fib_bench,
    boxed_fn_bench,
    );
criterion_main!(benches);
