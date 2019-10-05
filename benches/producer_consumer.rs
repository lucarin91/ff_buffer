#[macro_use]
extern crate criterion;
use criterion::{black_box, BenchmarkId, Criterion};

use ::ff_buffer;
use std::thread;
fn producer_consumer_rust(n: u64) {
    let (s, r) = ff_buffer::build::<Option<i64>>();

    let t1 = thread::spawn(move || {
        for i in 0..n {
            let el = Box::new(Some(i as i64));
            s.push(el);
        }
        s.push(Box::new(None));
    });
    let t2 = thread::spawn(move || {
        let mut count: i64 = 0;
        for el in r.iter() {
            if let None = *el {
                break;
            }
            count += (*el).unwrap();
            // println!("{}", (*el).unwrap());
        }
        black_box(count);
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

#[link(name = "ffbuffer")]
extern "C" {
    fn bench_producer_consumer_cpp(n: u64) -> u64;
}
fn producer_consumer_cpp(n: u64) {
    unsafe { black_box(bench_producer_consumer_cpp(n)) };
}

fn bench_pc(c: &mut Criterion) {
    let mut group = c.benchmark_group("producer_consumer");
    for i in [10_000, 50_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("C++", i), i, |b, i| {
            b.iter(|| producer_consumer_cpp(*i))
        });
        group.bench_with_input(BenchmarkId::new("Rust", i), i, |b, i| {
            b.iter(|| producer_consumer_rust(*i))
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_pc
}
criterion_main!(benches);
