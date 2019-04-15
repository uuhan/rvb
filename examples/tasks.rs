extern crate rvb as v8;
use v8::v8::{
    Platform,
    Isolate,
};

pub fn main() {
    let _platform = Platform::New();
    let mut isolate = Isolate::New();

    isolate.enqueue_closure(|_isolate| {
        println!("Hello from enqueued closure. #1");
    });
    isolate.enqueue_closure(|_isolate| {
        println!("Hello from enqueued closure. #2");
    });
    isolate.enqueue_closure(|_isolate| {
        println!("Hello from enqueued closure. #3");
    });
    isolate.enqueue_closure(|_isolate| {
        println!("Hello from enqueued closure. #4");
    });
    isolate.enqueue_closure(|_isolate| {
        println!("Hello from enqueued closure. #5");
    });

    let policy = isolate.get_microtasks_policy();
    println!("default policy: {}", policy);

    isolate.run_microtasks();
}
