use anyhow::Context;
use fxhash::FxHashSet;

use crate::{DayResult, IntoDayResult};

use Direction::*;

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("there should be a newline")?;
    let height = input.len() / (width + 1);

    let mut seen = FxHashSet::default();
    let mut seen_dir = FxHashSet::default();

    traverse(0, 0, Right, input, width, height, &mut seen, &mut seen_dir);
    let p1 = seen.len();

    let mut p2 = 0;

    for i in 0..width {
        seen.clear();
        seen_dir.clear();
        traverse(i, 0, Down, input, width, height, &mut seen, &mut seen_dir);
        p2 = std::cmp::max(p2, seen.len());
        seen.clear();
        seen_dir.clear();
        traverse(
            i,
            height - 1,
            Up,
            input,
            width,
            height,
            &mut seen,
            &mut seen_dir,
        );
        p2 = std::cmp::max(p2, seen.len());
    }

    for j in 0..height {
        seen.clear();
        seen_dir.clear();
        traverse(0, j, Right, input, width, height, &mut seen, &mut seen_dir);
        p2 = std::cmp::max(p2, seen.len());
        seen.clear();
        seen_dir.clear();
        traverse(
            width - 1,
            j,
            Left,
            input,
            width,
            height,
            &mut seen,
            &mut seen_dir,
        );
        p2 = std::cmp::max(p2, seen.len());
    }

    (p1, p2).into_result()
}

type Vector = (usize, usize, Direction);

#[allow(clippy::too_many_arguments)]
fn traverse(
    mut x: usize,
    mut y: usize,
    mut direction: Direction,
    world: &[u8],
    width: usize,
    height: usize,
    seen: &mut FxHashSet<(usize, usize)>,
    seen_dir: &mut FxHashSet<Vector>,
) {
    seen.insert((x, y));
    if !seen_dir.insert((x, y, direction)) {
        return;
    }

    loop {
        match world[x + y * (width + 1)] {
            b'.' => {}
            b'|' => match direction {
                Up | Down => {}
                Left | Right => {
                    traverse(x, y, Up, world, width, height, seen, seen_dir);
                    traverse(x, y, Down, world, width, height, seen, seen_dir);
                    return;
                }
            },
            b'-' => match direction {
                Left | Right => {}
                Up | Down => {
                    traverse(x, y, Left, world, width, height, seen, seen_dir);
                    traverse(x, y, Right, world, width, height, seen, seen_dir);
                    return;
                }
            },
            b'/' => {
                direction = match direction {
                    Up => Right,
                    Down => Left,
                    Left => Down,
                    Right => Up,
                }
            }
            b'\\' => {
                direction = match direction {
                    Up => Left,
                    Down => Right,
                    Left => Up,
                    Right => Down,
                }
            }
            _ => unreachable!("lmao"),
        }

        let Some((_x, _y)) = (match direction {
            Up => y.checked_sub(1).map(|y| (x, y)),
            Down => y.checked_add(1).map(|y| (x, y)),
            Left => x.checked_sub(1).map(|x| (x, y)),
            Right => x.checked_add(1).map(|x| (x, y)),
        }) else {
            return;
        };

        if _x >= width || _y >= height {
            return;
        }

        x = _x;
        y = _y;

        seen.insert((x, y));
        if !seen_dir.insert((x, y, direction)) {
            return;
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::{days::day16::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day16_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((46, 51).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day16.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((7_884, 8_185).into_day_result(), solution);
    }
}
