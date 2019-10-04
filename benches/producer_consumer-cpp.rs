#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::black_box;

#[link(name = "ffbuffer")]
extern "C" {
    fn cpp_producer_consumer(n: u64) -> i32;
}

fn producer_consumer(n: u64) {
  unsafe{cpp_producer_consumer(n)};
}

fn bench_pc(c: &mut Criterion) {
    c.bench_function("pc-cpp 1_000_000", |b| b.iter(|| producer_consumer(black_box(1_000_000))));
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(10);
    targets=bench_pc
}
criterion_main!(benches);
