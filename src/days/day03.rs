use std::collections::HashMap;

use anyhow::Context;
use arrayvec::ArrayVec;
use fxhash::FxBuildHasher;
use itertools::Itertools;

use crate::{DayResult, IntoDayResult};

macro_rules! update {
    ($building_number:ident, $found_symbol:ident, $numbers:ident, $number:ident, $number_count:ident, $p1:ident) => {
        if $building_number && $found_symbol {
            $p1 += $number;
            $numbers.insert($number_count, $number);
        }
        $number = 0;
        $building_number = false;
        $found_symbol = false;
    };
}

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;

    let width = input
        .as_bytes()
        .iter()
        .position(|&b| b == b'\n')
        .context("expected a newline")?;

    let world = World {
        src: input.as_bytes(),
        width,
    };

    let mut building_number = false;
    let mut found_symbol = false;
    let mut number = 0;
    let mut number_count = 0;
    let mut numbers = HashMap::with_hasher(FxBuildHasher::default());
    let mut asterisks = HashMap::with_hasher(FxBuildHasher::default());

    for y in 0..(input.len() / (width + 1)) {
        for x in 0..width {
            let b = world.get(x, y).context("this is a known legal coord")?;
            if b.is_ascii_digit() {
                number_count += 1 & (!building_number) as usize;
                building_number = true;
                number = number * 10 + (b - b'0') as usize;
                for (i, j) in (-1_isize..=1).cartesian_product(-1_isize..=1) {
                    let Ok(nx) = usize::try_from((x as isize) + i) else {
                        continue;
                    };
                    let Ok(ny) = usize::try_from((y as isize) + j) else {
                        continue;
                    };
                    let Some(t) = world.get(nx, ny) else { continue };
                    if t == b'.' || t.is_ascii_digit() {
                        continue;
                    }
                    if t == b'*' {
                        let ids = asterisks
                            .entry(nx + ny * width)
                            .or_insert(ArrayVec::<_, 2>::new());
                        if !ids.contains(&number_count) {
                            ids.push(number_count);
                        }
                    }
                    found_symbol = true;
                }
            } else {
                update!(
                    building_number,
                    found_symbol,
                    numbers,
                    number,
                    number_count,
                    p1
                );
            }
        }
        update!(
            building_number,
            found_symbol,
            numbers,
            number,
            number_count,
            p1
        );
    }

    let p2 = asterisks
        .into_iter()
        .filter_map(|(_, number_counts)| {
            (number_counts.len() == 2).then(|| {
                number_counts
                    .into_iter()
                    .flat_map(|count| numbers.get(&count))
                    .product::<usize>()
            })
        })
        .sum::<usize>();

    (p1, p2).into_result()
}

struct World<'a> {
    src: &'a [u8],
    width: usize,
}

impl<'a> World<'a> {
    fn get(&self, w: usize, h: usize) -> Option<u8> {
        self.src
            .get(w + h * (self.width + 1))
            .cloned()
            .filter(|&b| b != b'\n')
    }
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
