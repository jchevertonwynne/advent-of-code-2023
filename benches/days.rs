use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    const INPUT: &str = include_str!("../input/day01.txt");
    c.bench_function("day 01", |b| {
        b.iter(|| advent_of_code_2023::days::day01::solve(black_box(INPUT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
