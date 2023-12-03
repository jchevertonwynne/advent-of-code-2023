use std::collections::HashMap;

use anyhow::Context;
use arrayvec::ArrayVec;
use fxhash::FxBuildHasher;

use crate::{DayResult, IntoDayResult};

macro_rules! update {
    ($building_number:ident, $x:ident, $y:ident, $num_start:ident, $num_end:ident, $number:ident, $input:ident, $asterisks:ident, $width:ident, $p1:ident) => {
        if $building_number {
            let mut found_symbol = false;
            for ny in $y.checked_sub(1).unwrap_or($y)..($y + 2) {
                for nx in $num_start.checked_sub(1).unwrap_or($num_start)..($num_end + 2) {
                    if ny == $y && nx >= $num_start && nx <= $num_end {
                        continue;
                    }

                    let Some(&t) = $input.get(nx + ny * ($width + 1)) else {
                        continue;
                    };
                    if t == b'\n' || t == b'.' || t.is_ascii_digit() {
                        continue;
                    }

                    if t == b'*' {
                        let ids = $asterisks
                            .entry(nx + ny * $width)
                            .or_insert(ArrayVec::<Number, 2>::new());
                        let num = Number {
                            $num_start,
                            $num_end,
                            $number,
                        };
                        if !ids.contains(&num) {
                            ids.push(num);
                        }
                    }

                    found_symbol = true;
                }
            }
            if found_symbol {
                $p1 += $number;
            }
        }
        $number = 0;
        $building_number = false;
    };
}

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();

    let mut p1 = 0;

    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("expected a newline")?;

    let mut num_start = 0;
    let mut num_end = 0;
    let mut building_number = false;
    let mut number = 0;
    let mut asterisks = HashMap::with_hasher(FxBuildHasher::default());

    for y in 0..(input.len() / (width + 1)) {
        for x in 0..width {
            let &b = input
                .get(x + y * (width + 1))
                .context("this should be a known legal coord")?;

            if b.is_ascii_digit() {
                if !building_number {
                    num_start = x;
                }
                building_number = true;
                number = number * 10 + (b - b'0') as usize;
                num_end = x;
            } else {
                update!(
                    building_number,
                    x,
                    y,
                    num_start,
                    num_end,
                    number,
                    input,
                    asterisks,
                    width,
                    p1
                );
            }
        }

        update!(
            building_number,
            x,
            y,
            num_start,
            num_end,
            number,
            input,
            asterisks,
            width,
            p1
        );
    }

    let p2 = asterisks
        .into_iter()
        .filter_map(|(_, number_counts)| {
            (number_counts.len() == 2).then(|| {
                number_counts
                    .into_iter()
                    .map(|n| n.number)
                    .product::<usize>()
            })
        })
        .sum::<usize>();

    (p1, p2).into_result()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Number {
    num_start: usize,
    num_end: usize,
    number: usize,
}

#[cfg(test)]
mod tests {
    use crate::{days::day03::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day03_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((4_361, 467_835).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day03.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((530_849, 84_900_879).into_day_result(), solution);
    }
}
