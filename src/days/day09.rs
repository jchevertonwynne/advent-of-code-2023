use anyhow::Context;
use itertools::Itertools;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut triangle = Vec::new();
    let mut layers = 0;

    let mut input = input.as_bytes();

    while !input.is_empty() {
        triangle.clear();

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
            triangle.push(n);
            let last = input[0];
            input = &input[1..];
            if last == b'\n' {
                break;
            }
        }
        layers = 1;

        loop {
            let mut final_row = true;
            let r = range(layers, layers, triangle.len());
            for a in r.start..(r.end - 1) {
                let val = triangle[a + 1] - triangle[a];
                final_row &= val == 0;
                triangle.push(val);
            }
            layers += 1;
            if final_row {
                break;
            }
        }

        let mut end = 0;
        let (p1_end, p2_front) = (1..=layers).fold((0, 0), |(p1_end, p2_front), layer| {
            let r = range(layers, layer, triangle.len());
            assert_eq!(end, r.start);
            end = r.end;
            let row = &triangle[r];
            println!("{:?} {}", row, row.len());
            (row[row.len() - 1] + p1_end, row[0] - p2_front)
        });

        println!("1= {} 2= {}", p1_end, p2_front);
        p1 += p1_end;
        p2 += p2_front;
    }

    (p1, p2).into_result()
}

fn triangle(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    (n * (n - 1)) / 2
}

fn range(layers: usize, row: usize, len: usize) -> std::ops::Range<usize> {
    let t = triangle(layers);
    let common = (len - t) / layers;
    let extra = layers - row;
    let start = common * (row - 1) + (t - triangle(layers - row + 1));
    start..(start + common + extra)
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
