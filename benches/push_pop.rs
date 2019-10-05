#[macro_use]
extern crate criterion;
use criterion::{black_box, BenchmarkId, Criterion};

use ::ff_buffer;

fn push_pop_rust(n: u64) {
    let (s, r) = ff_buffer::build::<Option<i64>>();

    for i in 0..n {
        let el = Box::new(Some(i as i64));
        s.push(el);
    }
    s.push(Box::new(None));
    let mut count = 0;
    for el in r.iter() {
        if let None = *el {
            break;
        }
        count += (*el).unwrap();
    }
    black_box(count);
}

#[link(name = "ffbuffer")]
extern "C" {
    fn bench_push_pop_cpp(n: u64) -> u64;
}
fn push_pop_cpp(n: u64) {
    unsafe { black_box(bench_push_pop_cpp(n)) };
}

fn bench_pp(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_pop");
    for i in [10_000, 50_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("C++", i), i, |b, i| {
            b.iter(|| push_pop_cpp(*i))
        });
        group.bench_with_input(BenchmarkId::new("Rust", i), i, |b, i| {
            b.iter(|| push_pop_rust(*i))
        });
    }
    group.finish();
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(10);
    targets=bench_pp
}
criterion_main!(benches);