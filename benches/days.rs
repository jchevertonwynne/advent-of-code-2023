use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench_day {
        ($day:tt) => {{
            const INPUT: &str = include_str!(concat!("../input/", stringify!($day), ".txt"));
            c.bench_function(stringify!($day), |b| {
                b.iter(|| advent_of_code_2023::days::$day::solve(black_box(INPUT)))
            });
            // const INPUT_TEST: &str =
            //     include_str!(concat!("../input/", stringify!($day), "_test.txt"));
            // c.bench_function(concat!(stringify!($day), " test"), |b| {
            //     b.iter(|| advent_of_code_2023::days::$day::solve(black_box(INPUT_TEST)))
            // });
        }};
        ($day:tt, is_test) => {{
            const INPUT: &str = include_str!(concat!("../input/", stringify!($day), ".txt"));
            c.bench_function(stringify!($day), |b| {
                b.iter(|| advent_of_code_2023::days::$day::solve(black_box(INPUT), false))
            });
            // const INPUT_TEST: &str =
            //     include_str!(concat!("../input/", stringify!($day), "_test.txt"));
            // c.bench_function(concat!(stringify!($day), " test"), |b| {
            //     b.iter(|| advent_of_code_2023::days::$day::solve(black_box(INPUT_TEST), true))
            // });
        }};
    }

    // bench_day!(day01);
    // bench_day!(day02);
    // bench_day!(day03);
    // bench_day!(day04, is_test);
    // bench_day!(day05);
    // bench_day!(day06);
    // bench_day!(day07);
    // bench_day!(day08, is_test);
    // bench_day!(day09);
    // bench_day!(day10);
    // bench_day!(day11, is_test);
    // bench_day!(day12);
    // bench_day!(day13);
    // bench_day!(day14);
    // bench_day!(day15);
    // bench_day!(day16);
    bench_day!(day17);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
