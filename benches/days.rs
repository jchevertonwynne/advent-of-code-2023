use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench_day {
        ($day:tt) => {
            bench_day!($day, $day);
        };
        ($day:expr, $daymod:ident) => {{
            const INPUT: &str = include_str!(concat!("../input/", stringify!($day), ".txt"));
            c.bench_function(stringify!($day), |b| {
                b.iter(|| advent_of_code_2023::days::$daymod::solve(black_box(INPUT)))
            });
            const INPUT_TEST: &str =
                include_str!(concat!("../input/", stringify!($day), "_test.txt"));
            c.bench_function(concat!(stringify!($day), " test"), |b| {
                b.iter(|| advent_of_code_2023::days::$daymod::solve(black_box(INPUT_TEST)))
            });
        }};
    }

    bench_day!(day01);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
