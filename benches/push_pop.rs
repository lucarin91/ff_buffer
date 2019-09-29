#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::black_box;

use ::ff_buffer;

fn push_pop(n: u64) {
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

fn bench_pc(c: &mut Criterion) {
    c.bench_function("pp 1_000_000", |b| b.iter(|| push_pop(black_box(1_000_000))));
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(10);
    targets=bench_pc
}
criterion_main!(benches);
