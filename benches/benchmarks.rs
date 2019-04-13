#[macro_use]
extern crate criterion;
extern crate rand;

use count_sort;
use criterion::{Criterion, ParameterizedBenchmark};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "u8",
        ParameterizedBenchmark::new(
            "sort_u8",
            |b: &mut criterion::Bencher, &&length| {
                let mut rng = rand::thread_rng();
                let mut to_sort: Vec<Vec<u8>> = Vec::with_capacity(10);
                for _ in 0..10 {
                    let mut now: Vec<u8> = Vec::with_capacity(length);
                    for _ in 0..length {
                        now.push(rng.gen());
                    }
                    to_sort.push(now);
                }
                b.iter(|| {
                    for unsorted in to_sort.iter() {
                        let mut now = unsorted.clone();
                        count_sort::sort_u8(&mut now);
                        criterion::black_box(&now);
                    }
                })
            },
            &[0, 1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144],
        )
        .with_function("default sort", |b: &mut criterion::Bencher, &&length| {
            let mut rng = rand::thread_rng();
            let mut to_sort: Vec<Vec<u8>> = Vec::with_capacity(10);
            for _ in 0..10 {
                let mut now: Vec<u8> = Vec::with_capacity(length);
                for _ in 0..length {
                    now.push(rng.gen());
                }
                to_sort.push(now);
            }
            b.iter(|| {
                for unsorted in to_sort.iter() {
                    let mut now = unsorted.clone();
                    now.sort();
                    criterion::black_box(&now);
                }
            });
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
