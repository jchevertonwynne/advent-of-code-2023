use anyhow::Context;

use crate::{DayResult, IntoDayResult};

use Direction::*;

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("there should be a newline")?;
    let height = input.len() / (width + 1);

    let mut seen = vec![0_u8; width * height];

    traverse(0, 0, Right, input, width, height, &mut seen);
    let p1 = seen.iter().filter(|&&v| v != 0).count();

    let mut p2 = 0;

    for i in 0..width {
        seen.iter_mut().for_each(|v| *v = 0);
        traverse(i, 0, Down, input, width, height, &mut seen);
        p2 = std::cmp::max(p2, seen.iter().filter(|&&v| v != 0).count());
        seen.iter_mut().for_each(|v| *v = 0);
        traverse(i, height - 1, Up, input, width, height, &mut seen);
        p2 = std::cmp::max(p2, seen.iter().filter(|&&v| v != 0).count());
    }

    for j in 0..height {
        seen.iter_mut().for_each(|v| *v = 0);
        traverse(0, j, Right, input, width, height, &mut seen);
        p2 = std::cmp::max(p2, seen.iter().filter(|&&v| v != 0).count());
        seen.iter_mut().for_each(|v| *v = 0);
        traverse(width - 1, j, Left, input, width, height, &mut seen);
        p2 = std::cmp::max(p2, seen.iter().filter(|&&v| v != 0).count());
    }

    (p1, p2).into_result()
}

#[allow(clippy::too_many_arguments)]
fn traverse(
    mut x: usize,
    mut y: usize,
    mut direction: Direction,
    world: &[u8],
    width: usize,
    height: usize,
    seen: &mut Vec<u8>,
) {
    let d: u8 = direction.into();
    if seen[x + y * width] & d != 0 {
        return;
    }
    seen[x + y * width] |= d;

    loop {
        match world[x + y * (width + 1)] {
            b'.' => {}
            b'|' => match direction {
                Up | Down => {}
                Left | Right => {
                    traverse(x, y, Up, world, width, height, seen);
                    traverse(x, y, Down, world, width, height, seen);
                    return;
                }
            },
            b'-' => match direction {
                Left | Right => {}
                Up | Down => {
                    traverse(x, y, Left, world, width, height, seen);
                    traverse(x, y, Right, world, width, height, seen);
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

        let d: u8 = direction.into();
        if seen[x + y * width] & d != 0 {
            return;
        }
        seen[x + y * width] |= d;
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Up => 1 << 0,
            Down => 1 << 1,
            Left => 1 << 2,
            Right => 1 << 3,
        }
    }
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
