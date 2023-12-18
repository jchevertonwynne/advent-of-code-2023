use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Context;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("failed to find newline")?;
    let height = input.len() / (width + 1);
    let end = (width - 1, height - 1);

    let mut visited = vec![Lens::new(); width * height];
    let mut states = BinaryHeap::from_iter([Reverse(State::new((0, 0), 0, None, width, height))]);

    let p1 = solver::<1, 3>(&mut states, &mut visited, width, height, end, input);

    visited.iter_mut().for_each(|v| *v = Lens::new());
    states.clear();
    states.push(Reverse(State::new((0, 0), 0, None, width, height)));

    let p2 = solver::<4, 10>(&mut states, &mut visited, width, height, end, input);

    (p1, p2).into_result()
}

fn solver<const MOVE_MIN: u16, const MOVE_MAX: u16>(
    states: &mut BinaryHeap<Reverse<State>>,
    visited: &mut [Lens],
    width: usize,
    height: usize,
    end: (usize, usize),
    input: &[u8],
) -> usize {
    while let Some(Reverse(State {
        coord: coord @ (x, y),
        score,
        direction,
        ..
    })) = states.pop()
    {
        if let Some((direction, dist)) = direction {
            let l = &mut visited[x + y * width];
            match direction {
                Direction::Up => {
                    if l.up < dist {
                        continue;
                    }
                    l.up = std::cmp::min(l.up, dist);
                }
                Direction::Down => {
                    if l.down < dist {
                        continue;
                    }
                    l.down = std::cmp::min(l.down, dist);
                }
                Direction::Left => {
                    if l.left < dist {
                        continue;
                    }
                    l.left = std::cmp::min(l.left, dist);
                }
                Direction::Right => {
                    if l.right < dist {
                        continue;
                    }
                    l.right = std::cmp::min(l.right, dist);
                }
            }
        };

        if coord == end {
            return score;
        }

        match direction {
            Some((direction, dist)) => {
                if dist < MOVE_MAX {
                    if let Some(moved @ (x, y)) = coord.move_in(direction, width, height) {
                        if moved == end {
                            return score + (input[x + y * (width + 1)] - b'0') as usize;
                        }
                        if is_good(visited, direction, moved, width, dist + 1) {
                            states.push(Reverse(State::new(
                                moved,
                                score + (input[x + y * (width + 1)] - b'0') as usize,
                                Some((direction, dist + 1)),
                                width,
                                height,
                            )));
                        }
                    }
                }

                let left = direction.left();
                if let Some(value) = move_and_solve::<MOVE_MIN, MOVE_MAX>(
                    coord, left, width, height, input, end, score, visited, states,
                ) {
                    return value;
                }

                let right = direction.right();
                if let Some(value) = move_and_solve::<MOVE_MIN, MOVE_MAX>(
                    coord, right, width, height, input, end, score, visited, states,
                ) {
                    return value;
                }
            }
            None => {
                let down = Direction::Down;
                if let Some(value) = move_and_solve::<MOVE_MIN, MOVE_MAX>(
                    coord, down, width, height, input, end, score, visited, states,
                ) {
                    return value;
                }

                let right = Direction::Right;
                if let Some(value) = move_and_solve::<MOVE_MIN, MOVE_MAX>(
                    coord, right, width, height, input, end, score, visited, states,
                ) {
                    return value;
                }
            }
        }

        if let Some((direction, dist)) = direction {
            let l = &mut visited[x + y * width];
            match direction {
                Direction::Up => l.up = dist - 1,
                Direction::Down => l.down = dist - 1,
                Direction::Left => l.left = dist - 1,
                Direction::Right => l.right = dist - 1,
            }
        }
    }

    unreachable!("lmaoooooo")
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn move_and_solve<const MOVE_MIN: u16, const MOVE_MAX: u16>(
    coord: (usize, usize),
    left: Direction,
    width: usize,
    height: usize,
    input: &[u8],
    end: (usize, usize),
    score: usize,
    visited: &mut [Lens],
    states: &mut BinaryHeap<Reverse<State>>,
) -> Option<usize> {
    if let Some((moved, extra_score)) =
        (0..MOVE_MIN).try_fold((coord, 0), |(coord, extra_score), _| {
            coord
                .move_in(left, width, height)
                .map(|new_coord @ (x, y)| {
                    (
                        new_coord,
                        extra_score + (input[x + y * (width + 1)] - b'0') as usize,
                    )
                })
        })
    {
        if moved == end {
            return Some(score + extra_score);
        }
        if is_good(visited, left, moved, width, MOVE_MIN) {
            states.push(Reverse(State::new(
                moved,
                score + extra_score,
                Some((left, MOVE_MIN)),
                width,
                height,
            )));
        }
    }
    None
}

#[inline(always)]
fn is_good(
    lenses: &[Lens],
    direction: Direction,
    (x, y): (usize, usize),
    width: usize,
    to_beat: u16,
) -> bool {
    let l = &lenses[x + y * width];
    match direction {
        Direction::Up => l.up >= to_beat,
        Direction::Down => l.down >= to_beat,
        Direction::Left => l.left >= to_beat,
        Direction::Right => l.right >= to_beat,
    }
}

#[derive(Debug, Clone, Copy)]
struct Lens {
    up: u16,
    down: u16,
    left: u16,
    right: u16,
}

impl Lens {
    const fn new() -> Lens {
        Lens {
            up: u16::MAX,
            down: u16::MAX,
            left: u16::MAX,
            right: u16::MAX,
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    coord: (usize, usize),
    score: usize,
    direction: Option<(Direction, u16)>,
    est_dist: usize,
}

impl State {
    fn new(
        coord @ (x, y): (usize, usize),
        score: usize,
        direction: Option<(Direction, u16)>,
        width: usize,
        height: usize,
    ) -> State {
        State {
            coord,
            score,
            direction,
            est_dist: x.abs_diff(width - 1) + y.abs_diff(height - 1),
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
