#[macro_use]
extern crate criterion;
use criterion::black_box;
use criterion::Criterion;

use ::ff_buffer;
use std::thread;
fn producer_consumer(n: u64) {
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

fn bench_pc(c: &mut Criterion) {
    c.bench_function("pc-rust 1_000_000", |b| {
        b.iter(|| producer_consumer(black_box(1_000_000)))
    });
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(10);
    targets=bench_pc
}
criterion_main!(benches);
