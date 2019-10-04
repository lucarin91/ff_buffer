#[macro_use]
extern crate criterion;
use criterion::black_box;
use criterion::Criterion;

#[link(name = "ffbuffer")]
extern "C" {
    fn cpp_push_pop(n: u64) -> i32;
}

fn push_pop(n: u64) {
   unsafe{ cpp_push_pop(n) };
}

fn bench_pc(c: &mut Criterion) {
    c.bench_function("pp-cpp 1_000_000", |b| {
        b.iter(|| push_pop(black_box(1_000_000)))
    });
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(10);
    targets=bench_pc
}
criterion_main!(benches);
