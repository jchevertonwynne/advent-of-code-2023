use std::collections::{HashMap, HashSet};

use fxhash::FxBuildHasher;
use itertools::Itertools;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;

    let world = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut building_number = false;
    let mut found_symbol = false;
    let mut number = 0;
    let mut number_count = 0;
    let mut numbers = HashMap::with_hasher(FxBuildHasher::default());
    let mut asterisks = HashMap::with_hasher(FxBuildHasher::default());

    for (y, line) in world.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            if b.is_ascii_digit() {
                if !building_number {
                    number_count += 1;
                }
                building_number = true;
                number = number * 10 + (b - b'0') as usize;
                for (i, j) in (-1_isize..=1).cartesian_product(-1_isize..=1) {
                    let Ok(nx) = usize::try_from((x as isize) + i) else {
                        continue;
                    };
                    let Ok(ny) = usize::try_from((y as isize) + j) else {
                        continue;
                    };
                    if let Some(&t) = world.get(ny).and_then(|l| l.get(nx)) {
                        if t != b'.' && !t.is_ascii_digit() {
                            if t == b'*' {
                                asterisks
                                    .entry((nx, ny))
                                    .or_insert(HashSet::with_hasher(FxBuildHasher::default()))
                                    .insert(number_count);
                            }
                            found_symbol = true;
                        }
                    }
                }
            } else {
                if building_number && found_symbol {
                    p1 += number;
                    numbers.insert(number_count, number);
                }
                number = 0;
                building_number = false;
                found_symbol = false;
            }
        }
        if building_number && found_symbol {
            p1 += number;
            numbers.insert(number_count, number);
        }
        number = 0;
        building_number = false;
        found_symbol = false;
    }

    let p2 = asterisks
        .into_iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, number_counts)| {
            number_counts
                .into_iter()
                .flat_map(|count| numbers.get(&count))
                .product::<usize>()
        })
        .sum::<usize>();

    (p1, p2).into_result()
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
