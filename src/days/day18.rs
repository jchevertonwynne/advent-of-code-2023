use anyhow::Context;
use fxhash::FxHashSet;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut trench = FxHashSet::default();
    trench.insert((0_isize, 0_isize));
    let mut first_dir = None;
    let mut last_dir: Option<Direction> = None;

    let mut lr = 0_isize;

    for line in input.lines() {
        let (_, (direction, distance, _)) =
            parse_line(line).map_err(|err| anyhow::anyhow!("{err}"))?;

        first_dir = first_dir.or(Some(direction));
        if let Some(last_dir) = last_dir {
            debug_assert!(last_dir != direction);
            debug_assert!(last_dir != direction.right().right());

            if last_dir.left() == direction {
                lr -= 1;
            } else {
                lr += 1;
            }
        }

        let (dx, dy): (isize, isize) = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..distance {
            x += dx;
            y += dy;
            trench.insert((x, y));
        }

        last_dir = Some(direction);
    }

    let first_dir = first_dir.context("expected an intiail direction")?;
    let last_dir = last_dir.context("expected a final direction")?;
    if last_dir.left() == first_dir {
        lr -= 1;
    } else {
        lr += 1;
    }

    let mut internal: FxHashSet<(isize, isize)> = FxHashSet::default();
    let (sx, sy) = if lr > 0 {
        match first_dir {
            Direction::Up => (1, 1),
            Direction::Down => (-1, -1),
            Direction::Left => (-1, 1),
            Direction::Right => (1, -1),
        }
    } else {
        match first_dir {
            Direction::Up => (-1, 1),
            Direction::Down => (1, -1),
            Direction::Left => (-1, -1),
            Direction::Right => (1, 1),
        }
    };
    flood_fill(sx, sy, &trench, &mut internal);

    let p1 = internal.len() + trench.len();

    (p1).into_result()
}

fn flood_fill(
    x: isize,
    y: isize,
    trench: &FxHashSet<(isize, isize)>,
    inner: &mut FxHashSet<(isize, isize)>,
) {
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if !trench.contains(&(nx, ny)) && inner.insert((nx, ny)) {
            flood_fill(nx, ny, trench, inner);
        }
    }
}

fn parse_direction(line: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("U"), |_| Direction::Up),
        map(tag("D"), |_| Direction::Down),
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
    ))(line)
}

fn parse_hex(line: &str) -> IResult<&str, [u8; 3]> {
    map(
        nom::combinator::map_res(nom::character::complete::hex_digit1, |h| {
            u32::from_str_radix(h, 16)
        }),
        |digit| [(digit >> 16) as u8, (digit >> 8) as u8, digit as u8],
    )(line)
}

fn parse_line(line: &str) -> IResult<&str, (Direction, u32, [u8; 3])> {
    map(
        nom::sequence::tuple((
            parse_direction,
            tag(" "),
            nom::character::complete::u32,
            tag(" (#"),
            parse_hex,
            tag(")"),
        )),
        |(direction, _, distance, _, hex, _)| (direction, distance, hex),
    )(line)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn right(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day18::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day18_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(().into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day18.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(().into_day_result(), solution);
    }
}
