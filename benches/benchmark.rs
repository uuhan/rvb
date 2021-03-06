#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::ParameterizedBenchmark;
use criterion::black_box;
use std::mem;
use rvb::v8::{
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

fn normal_closure() {
    let closure = || {};
    closure()
}

fn boxed_closure() {
    let closure = || {};
    let closure: Box<Box<FnMut()>>
        = Box::new(Box::new(closure));

    let closure: &mut Box<FnMut()> = unsafe { mem::transmute(closure) };
    closure()
}

fn boxed_closure_bench(c: &mut Criterion) {
    c.bench(
        "closure",
        ParameterizedBenchmark::new(
            "boxed closure",
            |b, _| b.iter(|| boxed_closure()),
            vec![1,2,3])
        .with_function(
            "normal closure",
            |b, _| b.iter(|| normal_closure())
        )
    );
}

criterion_group!(
    benches,
    fib_bench,
    boxed_closure_bench,
    );
criterion_main!(benches);
