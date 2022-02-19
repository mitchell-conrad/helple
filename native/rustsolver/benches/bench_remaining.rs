use criterion::{criterion_group, criterion_main, Criterion};
use rustsolver::{remaining_wordles, calc};
use std::iter::FromIterator;

fn remaining_benchmark(c: &mut Criterion) {
    c.bench_function("remaining wordles: swill", |b| {
        b.iter(|| remaining_wordles(Vec::from_iter("treadbok".chars()), &vec!(('s', 4)),
        &vec!(('i', 2), ('l', 3), ('s', 0))))


    });
}

fn calc_benchmark(c: &mut Criterion) {
    c.bench_function("calc wordles: shake", |b| {
        b.iter(|| {
            calc(
                "shake",
                vec![
                    "suite".to_string(),
                    "snare".to_string(),
                    "spade".to_string(),
                    "shame".to_string(),
                    "shale".to_string(),
                ],
            )
        })
    });
}
criterion_group!(benches, remaining_benchmark, calc_benchmark);
criterion_main!(benches);
