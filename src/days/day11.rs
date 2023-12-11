use anyhow::Context;
use itertools::Itertools;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("should be a newline")?;
    let height = input.len() / (width + 1);

    let mut accums = (0..height)
        .map(|h| (0..width).all(|w| input[w + h * (width + 1)] == b'.') as usize)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let vert_count = accums.len();

    (0..width)
        .map(|w| (0..height).all(|h| input[w + h * (width + 1)] == b'.') as usize)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .for_each(|a| accums.push(a));

    let vert_accum = &accums[..vert_count];
    let hori_accum = &accums[vert_count..];

    let galaxies = (0..height)
        .flat_map(|h| {
            (0..width)
                .filter(move |&w| input[w + h * (width + 1)] == b'#')
                .map(move |w| (w, h))
        })
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    let multiplier = if is_test { 10 } else { 1_000_000 };

    for (&(i1, j1), &(i2, j2)) in galaxies.iter().tuple_combinations() {
        let dist = i1.abs_diff(i2) + j1.abs_diff(j2);
        let i_min = std::cmp::min(i1, i2);
        let i_max = std::cmp::max(i1, i2);
        let j_min = std::cmp::min(j1, j2);
        let j_max = std::cmp::max(j1, j2);
        let additive =
            hori_accum[i_max] - hori_accum[i_min] + vert_accum[j_max] - vert_accum[j_min];

        p1 += dist + additive;
        p2 += dist + (additive * (multiplier - 1));
    }

    (p1, p2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day11::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day11_test.txt");
        let solution = solve(INPUT, true).unwrap();
        assert_eq!((374, 1_030).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day11.txt");
        let solution = solve(INPUT, false).unwrap();
        assert_eq!(
            (9_623_138, 726_820_169_514_usize).into_day_result(),
            solution
        );
    }
}
