use std::ops::ControlFlow;

use anyhow::Context;
use fxhash::FxHashSet;
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("failed to find newline")?;
    let world = TileMap {
        raw: input,
        width: width + 1,
    };

    let start = input
        .iter()
        .position(|&b| b == b'S')
        .map(|i| (i % (width + 1), i / (width + 1)))
        .context("there should be a start tile")?;

    let mut start_tile = Tile::Start;

    let mut pipe_tiles = FxHashSet::default();
    let mut p1 = 0;
    'd: for mut dir in Direction::iter() {
        let start_dir = dir;
        let (mut x, mut y) = start;
        let mut moves = 0;

        pipe_tiles.clear();
        pipe_tiles.insert((x, y));

        loop {
            let Some((nx, ny)) = (match dir {
                Direction::Up => y.checked_sub(1).map(|y| (x, y)),
                Direction::Down => y.checked_add(1).map(|y| (x, y)),
                Direction::Left => x.checked_sub(1).map(|x| (x, y)),
                Direction::Right => x.checked_add(1).map(|x| (x, y)),
            }) else {
                break;
            };

            let next_tile = world.get(nx, ny);
            if let ControlFlow::Break(break_inner) = process_tile(
                next_tile,
                &mut dir,
                &mut start_tile,
                start_dir,
                &mut moves,
                &mut p1,
            ) {
                if break_inner {
                    break;
                } else {
                    break 'd;
                }
            }

            x = nx;
            y = ny;
            moves += 1;
            pipe_tiles.insert((x, y));
        }
    }

    // from the row of the start tile scan inwards until the pipes are hit & declare that side as Outer
    // then traverse the pipe & declare the opposite side as inner

    let mut x = 0;
    let mut y = start.1;

    while !pipe_tiles.contains(&(x, y)) {
        x += 1;
    }

    let new_start = (x, y);

    let mut contained = FxHashSet::default();

    let tile = world.get(x, y);
    // we can only be on a vertival or a left corner
    // if we're on a vertical tile - read to right
    // else if corner move right/up
    let mut dir = find_start_dir(tile, start_tile);
    loop {
        let tile = world.get(x, y);
        follow_pipe_and_flood_fill(
            tile,
            &mut dir,
            &mut x,
            &mut y,
            &pipe_tiles,
            &mut contained,
            start_tile,
        );

        if (x, y) == new_start {
            break;
        }
    }

    let p2 = contained.len();

    (p1, p2).into_result()
}

fn process_tile(
    next_tile: Tile,
    dir: &mut Direction,
    start_tile: &mut Tile,
    start_dir: Direction,
    moves: &mut i32,
    p1: &mut i32,
) -> ControlFlow<bool> {
    match next_tile {
        Tile::Vertical => {
            if matches!(*dir, Direction::Up | Direction::Down) {
            } else {
                return ControlFlow::Break(true);
            }
        }
        Tile::Horizontal => {
            if matches!(*dir, Direction::Left | Direction::Right) {
            } else {
                return ControlFlow::Break(true);
            }
        }
        Tile::CornerL => {
            if matches!(*dir, Direction::Left | Direction::Down) {
                *dir = if *dir == Direction::Left {
                    Direction::Up
                } else {
                    Direction::Right
                }
            } else {
                return ControlFlow::Break(true);
            }
        }
        Tile::CornerJ => {
            if matches!(*dir, Direction::Right | Direction::Down) {
                *dir = if *dir == Direction::Right {
                    Direction::Up
                } else {
                    Direction::Left
                }
            } else {
                return ControlFlow::Break(true);
            }
        }
        Tile::Corner7 => {
            if matches!(*dir, Direction::Right | Direction::Up) {
                *dir = if *dir == Direction::Right {
                    Direction::Down
                } else {
                    Direction::Left
                }
            } else {
                return ControlFlow::Break(true);
            }
        }
        Tile::CornerF => {
            if matches!(*dir, Direction::Left | Direction::Up) {
                *dir = if *dir == Direction::Left {
                    Direction::Down
                } else {
                    Direction::Right
                }
            } else {
                return ControlFlow::Break(true);
            }
        }
        Tile::Start => {
            *start_tile = find_pipe_type(start_dir, *dir);
            *moves += 1;
            *p1 = *moves / 2;
            return ControlFlow::Break(false);
        }
        Tile::Empty => return ControlFlow::Break(true),
    }
    ControlFlow::Continue(())
}

fn find_pipe_type(start_dir: Direction, dir: Direction) -> Tile {
    match (start_dir, dir) {
        (Direction::Up, Direction::Up) => Tile::Vertical,
        (Direction::Up, Direction::Left) => Tile::CornerL,
        (Direction::Up, Direction::Right) => Tile::CornerJ,

        (Direction::Down, Direction::Down) => Tile::Vertical,
        (Direction::Down, Direction::Left) => Tile::CornerF,
        (Direction::Down, Direction::Right) => Tile::Corner7,

        (Direction::Left, Direction::Up) => Tile::Corner7,
        (Direction::Left, Direction::Down) => Tile::CornerJ,
        (Direction::Left, Direction::Left) => Tile::Horizontal,

        (Direction::Right, Direction::Up) => Tile::CornerF,
        (Direction::Right, Direction::Down) => Tile::CornerL,
        (Direction::Right, Direction::Right) => Tile::Horizontal,
        _ => unreachable!("lmao"),
    }
}

fn follow_pipe_and_flood_fill(
    tile: Tile,
    dir: &mut Direction,
    x: &mut usize,
    y: &mut usize,
    pipe_tiles: &FxHashSet<(usize, usize)>,
    contained: &mut FxHashSet<(usize, usize)>,
    start_tile: Tile,
) {
    match tile {
        Tile::Vertical => {
            if *dir == Direction::Up {
                flood_fill(*x + 1, *y, pipe_tiles, contained);
                *y -= 1;
            } else {
                flood_fill(*x - 1, *y, pipe_tiles, contained);
                *y += 1;
            }
        }
        Tile::Horizontal => {
            if *dir == Direction::Right {
                flood_fill(*x, *y + 1, pipe_tiles, contained);
                *x += 1;
            } else {
                flood_fill(*x, *y - 1, pipe_tiles, contained);
                *x -= 1;
            }
        }
        Tile::Corner7 => {
            if *dir == Direction::Up {
                flood_fill(*x, *y - 1, pipe_tiles, contained);
                flood_fill(*x + 1, *y, pipe_tiles, contained);
                *dir = Direction::Left;
                *x -= 1;
            } else {
                *dir = Direction::Down;
                *y += 1;
            }
        }
        Tile::CornerJ => {
            if *dir == Direction::Down {
                *dir = Direction::Left;
                *x -= 1;
            } else {
                flood_fill(*x, *y + 1, pipe_tiles, contained);
                flood_fill(*x + 1, *y, pipe_tiles, contained);
                *dir = Direction::Up;
                *y -= 1;
            }
        }
        Tile::CornerL => {
            if *dir == Direction::Down {
                flood_fill(*x, *y + 1, pipe_tiles, contained);
                flood_fill(*x - 1, *y, pipe_tiles, contained);
                *dir = Direction::Right;
                *x += 1;
            } else {
                *dir = Direction::Up;
                *y -= 1;
            }
        }
        Tile::CornerF => {
            if *dir == Direction::Up {
                *dir = Direction::Right;
                *x += 1;
            } else {
                flood_fill(*x, *y - 1, pipe_tiles, contained);
                flood_fill(*x - 1, *y, pipe_tiles, contained);
                *dir = Direction::Down;
                *y += 1;
            }
        }
        Tile::Start => {
            follow_pipe_and_flood_fill(start_tile, dir, x, y, pipe_tiles, contained, start_tile)
        }
        _ => unreachable!("found a {tile:?} at {x} {y}"),
    };
}

fn find_start_dir(tile: Tile, start_tile: Tile) -> Direction {
    match tile {
        Tile::Vertical => Direction::Up,
        Tile::CornerL => Direction::Left,
        Tile::CornerF => Direction::Up,
        Tile::Start => find_start_dir(start_tile, start_tile),
        _ => unreachable!("found a {tile:?}"),
    }
}

fn flood_fill(
    x: usize,
    y: usize,
    path: &FxHashSet<(usize, usize)>,
    contained: &mut FxHashSet<(usize, usize)>,
) {
    if path.contains(&(x, y)) {
        return;
    }
    if contained.insert((x, y)) {
        flood_fill(x - 1, y, path, contained);
        flood_fill(x + 1, y, path, contained);
        flood_fill(x, y - 1, path, contained);
        flood_fill(x, y + 1, path, contained);
    }
}

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    CornerL,
    CornerJ,
    Corner7,
    CornerF,
    Start,
    Empty,
}

struct TileMap<'a> {
    raw: &'a [u8],
    width: usize,
}

impl<'a> TileMap<'a> {
    fn get(&self, x: usize, y: usize) -> Tile {
        self.raw
            .get(x + y * self.width)
            .map(|&b| match b {
                b'|' => Tile::Vertical,
                b'-' => Tile::Horizontal,
                b'L' => Tile::CornerL,
                b'J' => Tile::CornerJ,
                b'7' => Tile::Corner7,
                b'F' => Tile::CornerF,
                b'.' => Tile::Empty,
                b'S' => Tile::Start,
                b'\n' => Tile::Empty,
                _ => unreachable!("please handle me: {}", b as char),
            })
            .unwrap_or(Tile::Empty)
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day10::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day10_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((80, 10).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day10.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((6_882, 491).into_day_result(), solution);
    }
}
