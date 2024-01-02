use std::convert::Infallible;

use anyhow::Context;
use fxhash::FxHashSet;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let width = input
        .bytes()
        .position(|b| b == b'\n')
        .context("expected a newline")?;
    let height = input.len() / (width + 1);
    let width = width as isize;
    let height = height as isize;

    let mut start = (0, 0);
    let mut world = FxHashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            let (x, y) = (x as isize, y as isize);
            if b == b'#' {
                world.insert((x, y));
            } else if b == b'S' {
                start = (x, y);
            }
        }
    }

    let turns_1 = if is_test { 6 } else { 64 };
    let turns_2 = if is_test { 6 } else { 26_501_365 };

    let p1 = solve_1(&world, start, turns_1);
    let p2 = solve_2(&world, start, turns_2, width, height);

    (p1, p2).into_result()
}

fn solve_1(world: &FxHashSet<(isize, isize)>, start: (isize, isize), turns: usize) -> usize {
    let mut states = FxHashSet::from_iter([start]);
    let mut states_swap = FxHashSet::default();
    for _ in 0..turns {
        states_swap.clear();

        for (x, y) in states.drain() {
            for next_state in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if world.contains(&next_state) {
                    continue;
                }
                states_swap.insert(next_state);
            }
        }

        std::mem::swap(&mut states, &mut states_swap);
    }

    states.len()
}

fn solve_2(
    world: &FxHashSet<(isize, isize)>,
    (sx, sy): (isize, isize),
    turns: isize,
    width: isize,
    height: isize,
) -> usize {
    let mut res = 0;

    for y in (-turns)..(turns + 1) {
        let w = (turns - y.abs()).abs();
        for x in ((-w)..(w + 1)).step_by(2) {
            let rx = sx + x;
            let ry = sy + y;
            let nx = rx.rem_euclid(width);
            let ny = ry.rem_euclid(height);
            println!("raw = {}, {} rem = {}, {}", rx, ry, nx, ny);
            if !world.contains(&(nx, ny)) {
                res += 1;
                println!("found on grid {} {}", nx, ny);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{days::day21::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day21_test.txt");
        let solution = solve(INPUT, true).unwrap();
        assert_eq!(().into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day21.txt");
        let solution = solve(INPUT, false).unwrap();
        assert_eq!(().into_day_result(), solution);
    }
}
