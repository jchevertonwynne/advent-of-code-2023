use anyhow::Context;
use itertools::Itertools;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let pool = object_pool::Pool::new(4, Vec::<isize>::new);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut triangle = Vec::new();

    let mut input = input.as_bytes();

    while !input.is_empty() {
        triangle.clear();
        let mut vd = pool.pull(Vec::new);
        vd.clear();

        while !input.is_empty() {
            let mut negative = false;
            let mut n = 0;
            loop {
                let b = input[0];
                if b == b'-' {
                    negative = true;
                } else if b.is_ascii_digit() {
                    n = n * 10 + (b - b'0') as isize;
                } else {
                    break;
                }
                input = &input[1..];
            }
            if negative {
                n *= -1;
            }
            vd.push(n);
            let last = input[0];
            input = &input[1..];
            if last == b'\n' {
                break;
            }
        }
        triangle.push(vd);
        loop {
            let last = triangle.last().context("exp at least 1 row")?;
            let mut final_row = true;
            let mut vd = pool.pull(Vec::new);
            vd.clear();
            let next = last
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| {
                    let res = b - a;
                    final_row &= res == 0;
                    res
                })
                .fold(vd, |mut acc, v| {
                    acc.push(v);
                    acc
                });
            triangle.push(next);
            if final_row {
                break;
            }
        }

        let (p1_end, p2_front) = triangle.iter().rev().fold((0, 0), |(p1_end, p2_front), t| {
            (t[t.len() - 1] + p1_end, t[0] - p2_front)
        });
        p1 += p1_end;
        p2 += p2_front;
    }

    (p1, p2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day09::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day09_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((114, 2).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day09.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_939_607_039, 1_041).into_day_result(), solution);
    }
}
