#[macro_use]
extern crate criterion;
extern crate dmsort;
extern crate rand;

use count_sort;
use criterion::{Criterion, ParameterizedBenchmark};
use rand::Rng;

fn generate_to_sort(length: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut to_sort: Vec<Vec<u8>> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let mut now: Vec<u8> = Vec::with_capacity(length);
        for _ in 0..length {
            now.push(rng.gen());
        }
        to_sort.push(now);
    }
    to_sort
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "u8",
        ParameterizedBenchmark::new(
            "sort_u8",
            |b: &mut criterion::Bencher, &&length| {
                let mut pos = 0;
                let to_sort = generate_to_sort(length);
                b.iter(|| {
                    pos = (pos + 1) % 1000;
                    let mut now = to_sort[pos].clone();
                    count_sort::sort_u8(&mut now);
                    criterion::black_box(&now);
                })
            },
            &[0, 1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144],
        )
        .with_function("default sort", |b: &mut criterion::Bencher, &&length| {
            let mut pos = 0;
            let to_sort = generate_to_sort(length);
            b.iter(|| {
                pos = (pos + 1) % 1000;
                let mut now = to_sort[pos].clone();
                now.sort();
                criterion::black_box(&now);
            });
        })
        .with_function(
            "default sort_unstable",
            |b: &mut criterion::Bencher, &&length| {
                let mut pos = 0;
                let to_sort = generate_to_sort(length);
                b.iter(|| {
                    pos = (pos + 1) % 1000;
                    let mut now = to_sort[pos].clone();
                    now.sort_unstable();
                    criterion::black_box(&now);
                });
            },
        )
        .with_function("dmsort", |b: &mut criterion::Bencher, &&length| {
            let mut pos = 0;
            let to_sort = generate_to_sort(length);
            b.iter(|| {
                pos = (pos + 1) % 1000;
                let mut now = to_sort[pos].clone();
                dmsort::sort(&mut now);
                criterion::black_box(&now);
            });
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
