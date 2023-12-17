use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Context;
use fxhash::FxHashSet;

use crate::{DayResult, IntoDayResult};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {
    let input = _input.as_bytes();
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("failed to find newline")?;
    let height = input.len() / (width + 1);
    let end = (width - 1, height - 1);

    let mut visited = FxHashSet::default();
    let mut p1 = 0;
    let mut states = BinaryHeap::from_iter([Reverse(State::new((0, 0), 0, None, width, height))]);
    while let Some(Reverse(State {
        coord,
        score,
        direction,
        ..
    })) = states.pop()
    {
        if !visited.insert((coord, direction)) {
            continue;
        }
        if coord == end {
            p1 = score;
            break;
        }
        match direction {
            Some((direction, dist)) => {
                if dist < 3 {
                    if let Some(moved) = coord.move_in(direction, width, height) {
                        states.push(Reverse(State::new(
                            moved,
                            score + (input[moved.0 + moved.1 * (width + 1)] - b'0') as usize,
                            Some((direction, dist + 1)),
                            width,
                            height,
                        )));
                    }
                }

                let left = direction.left();
                if let Some(moved) = coord.move_in(left, width, height) {
                    states.push(Reverse(State::new(
                        moved,
                        score + (input[moved.0 + moved.1 * (width + 1)] - b'0') as usize,
                        Some((left, 1)),
                        width,
                        height,
                    )));
                }

                let right = direction.right();
                if let Some(moved) = coord.move_in(right, width, height) {
                    states.push(Reverse(State::new(
                        moved,
                        score + (input[moved.0 + moved.1 * (width + 1)] - b'0') as usize,
                        Some((right, 1)),
                        width,
                        height,
                    )));
                }
            }
            None => {
                let direction = Direction::Down;
                if let Some(moved) = coord.move_in(direction, width, height) {
                    states.push(Reverse(State::new(
                        moved,
                        score + (input[moved.0 + moved.1 * (width + 1)] - b'0') as usize,
                        Some((direction, 1)),
                        width,
                        height,
                    )));
                }

                let direction = Direction::Right;
                if let Some(moved) = coord.move_in(direction, width, height) {
                    states.push(Reverse(State::new(
                        moved,
                        score + (input[moved.0 + moved.1 * (width + 1)] - b'0') as usize,
                        Some((direction, 1)),
                        width,
                        height,
                    )));
                }
            }
        }
    }

    states.clear();
    visited.clear();
    let mut p2 = 0;
    states.push(Reverse(State::new((0, 0), 0, None, width, height)));
    while let Some(Reverse(State {
        coord,
        score,
        direction,
        ..
    })) = states.pop()
    {
        if !visited.insert((coord, direction)) {
            continue;
        }
        if coord == end {
            p2 = score;
            break;
        }
        match direction {
            Some((direction, dist)) => {
                if dist < 10 {
                    if let Some(moved) = coord.move_in(direction, width, height) {
                        states.push(Reverse(State::new(
                            moved,
                            score + (input[moved.0 + moved.1 * (width + 1)] - b'0') as usize,
                            Some((direction, dist + 1)),
                            width,
                            height,
                        )));
                    }
                }

                let left = direction.left();
                if let Some((moved, extra_score)) =
                    (0..4).try_fold((coord, 0), |(coord, extra_score), _| {
                        coord.move_in(left, width, height).map(|new_coord| {
                            (
                                new_coord,
                                extra_score
                                    + (input[new_coord.0 + new_coord.1 * (width + 1)] - b'0')
                                        as usize,
                            )
                        })
                    })
                {
                    states.push(Reverse(State::new(
                        moved,
                        score + extra_score,
                        Some((left, 4)),
                        width,
                        height,
                    )));
                }

                let right = direction.right();
                if let Some((moved, extra_score)) =
                    (0..4).try_fold((coord, 0), |(coord, extra_score), _| {
                        coord.move_in(right, width, height).map(|new_coord| {
                            (
                                new_coord,
                                extra_score
                                    + (input[new_coord.0 + new_coord.1 * (width + 1)] - b'0')
                                        as usize,
                            )
                        })
                    })
                {
                    states.push(Reverse(State::new(
                        moved,
                        score + extra_score,
                        Some((right, 4)),
                        width,
                        height,
                    )));
                }
            }
            None => {
                let direction = Direction::Down;
                if let Some((moved, extra_score)) =
                    (0..4).try_fold((coord, 0), |(coord, extra_score), _| {
                        coord.move_in(direction, width, height).map(|new_coord| {
                            (
                                new_coord,
                                extra_score
                                    + (input[new_coord.0 + new_coord.1 * (width + 1)] - b'0')
                                        as usize,
                            )
                        })
                    })
                {
                    states.push(Reverse(State::new(
                        moved,
                        score + extra_score,
                        Some((direction, 4)),
                        width,
                        height,
                    )));
                }

                let direction = Direction::Right;
                if let Some((moved, extra_score)) =
                    (0..4).try_fold((coord, 0), |(coord, extra_score), _| {
                        coord.move_in(direction, width, height).map(|new_coord| {
                            (
                                new_coord,
                                extra_score
                                    + (input[new_coord.0 + new_coord.1 * (width + 1)] - b'0')
                                        as usize,
                            )
                        })
                    })
                {
                    states.push(Reverse(State::new(
                        moved,
                        score + extra_score,
                        Some((direction, 4)),
                        width,
                        height,
                    )));
                }
            }
        }
    }

    (p1, p2).into_result()
}

#[derive(Eq, PartialEq)]
struct State {
    coord: (usize, usize),
    score: usize,
    direction: Option<(Direction, usize)>,
    est_dist: usize,
}

impl State {
    fn new(
        coord: (usize, usize),
        score: usize,
        direction: Option<(Direction, usize)>,
        width: usize,
        height: usize,
    ) -> State {
        State {
            coord,
            score,
            direction,
            est_dist: coord.0.abs_diff(width - 1) + coord.1.abs_diff(height - 1),
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.score + self.est_dist;
        let b = other.score + other.est_dist;
        a.cmp(&b)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

trait MoveIn: Sized {
    fn move_in(self, direction: Direction, width: usize, height: usize) -> Option<Self>;
}

impl MoveIn for (usize, usize) {
    fn move_in(self, direction: Direction, width: usize, height: usize) -> Option<Self> {
        let (x, y) = self;
        match direction {
            Direction::Up => y.checked_sub(1).map(|y| (x, y)),
            Direction::Down => y.checked_add(1).map(|y| (x, y)),
            Direction::Left => x.checked_sub(1).map(|x| (x, y)),
            Direction::Right => x.checked_add(1).map(|x| (x, y)),
        }
        .filter(|&(x, y)| x < width && y < height)
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day17::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day17_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((102, 94).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day17.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((861, 1_037).into_day_result(), solution);
    }
}
