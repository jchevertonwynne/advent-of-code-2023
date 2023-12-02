use arrayvec::ArrayVec;
use bstr::ByteSlice;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.as_bytes().lines() {
        let (p1f, p2f) = first_omni::<StateForward>(line);
        let (p1b, p2b) = first_omni::<StateBackward>(line);
        let num1 = p1f * 10 + p1b;
        let num2 = p2f * 10 + p2b;
        p1 += num1;
        p2 += num2;
    }

    (p1, p2).into_result()
}

fn first_omni<F>(mut line: &[u8]) -> (usize, usize)
where
    F: Feedable,
{
    let mut found_p1 = false;
    let mut found_p2 = false;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut a1: ArrayVec<F, 9> = ArrayVec::from_iter([F::init()]);

    while !line.is_empty() {
        let (b, _line) = F::incr(line);
        line = _line;

        match b {
            b @ b'1'..=b'9' => {
                a1 = ArrayVec::from_iter([F::init()]);

                let num = (b - b'0') as usize;
                p1 = num;
                found_p1 = true;
                if !found_p2 {
                    p2 = num;
                    found_p2 = true;
                }
            }
            b => {
                if found_p2 {
                    continue;
                }
                let mut a2 = ArrayVec::from_iter([F::init()]);

                while let Some(s) = a1.pop() {
                    match s.feed(b) {
                        FeedResult::None => {}
                        FeedResult::One(a) => a2.push(a),
                        FeedResult::Completeable(n, rem) => {
                            if F::completes(line, rem) {
                                p2 = n;
                                found_p2 = true;
                                break;
                            }
                        }
                    }
                }

                std::mem::swap(&mut a1, &mut a2);
            }
        }

        if found_p1 && found_p2 {
            break;
        }
    }

    (p1, p2)
}

trait Feedable: Copy {
    fn init() -> Self;
    fn feed(self, b: u8) -> FeedResult<Self>;
    fn incr(line: &[u8]) -> (u8, &[u8]);
    fn completes(line: &[u8], rem: &[u8]) -> bool;
}

enum FeedResult<S> {
    None,
    One(S),
    Completeable(usize, &'static [u8]),
}

#[derive(Debug, Clone, Copy)]
struct StateForward(u8);

impl Feedable for StateForward {
    fn init() -> Self {
        StateForward(0)
    }

    fn feed(self, b: u8) -> FeedResult<StateForward> {
        match self.0 {
            // first layer
            0 => match b {
                b'o' => FeedResult::Completeable(1, b"ne"),
                b't' => FeedResult::One(StateForward(2)),
                b'f' => FeedResult::One(StateForward(3)),
                b's' => FeedResult::One(StateForward(4)),
                b'e' => FeedResult::Completeable(8, b"ight"),
                b'n' => FeedResult::Completeable(9, b"ine"),
                _ => FeedResult::None,
            },
            // second layer
            1 => match b {
                b'n' => FeedResult::One(StateForward(7)),
                _ => FeedResult::None,
            },
            2 => match b {
                b'w' => FeedResult::Completeable(2, b"o"),
                b'h' => FeedResult::Completeable(3, b"ree"),
                _ => FeedResult::None,
            },
            3 => match b {
                b'o' => FeedResult::Completeable(4, b"ur"),
                b'i' => FeedResult::Completeable(5, b"ve"),
                _ => FeedResult::None,
            },
            4 => match b {
                b'i' => FeedResult::Completeable(6, b"x"),
                b'e' => FeedResult::Completeable(7, b"ven"),
                _ => FeedResult::None,
            },
            _ => FeedResult::None,
        }
    }

    fn completes(line: &[u8], rem: &[u8]) -> bool {
        line.starts_with(rem)
    }

    fn incr(line: &[u8]) -> (u8, &[u8]) {
        (line[0], &line[1..])
    }
}

#[derive(Debug, Clone, Copy)]
struct StateBackward(u8);

impl Feedable for StateBackward {
    fn init() -> Self {
        StateBackward(0)
    }

    fn feed(self, b: u8) -> FeedResult<StateBackward> {
        match self.0 {
            // first layer
            0 => match b {
                b'e' => FeedResult::One(StateBackward(1)),
                b'o' => FeedResult::Completeable(2, b"tw"),
                b'r' => FeedResult::Completeable(4, b"fou"),
                b'x' => FeedResult::Completeable(6, b"si"),
                b'n' => FeedResult::Completeable(7, b"seve"),
                b't' => FeedResult::Completeable(8, b"eigh"),
                _ => FeedResult::None,
            },
            // second layer
            1 => match b {
                b'v' => FeedResult::Completeable(5, b"fi"),
                b'n' => FeedResult::One(StateBackward(2)),
                b'e' => FeedResult::Completeable(3, b"thr"),
                _ => FeedResult::None,
            },
            2 => match b {
                b'o' => FeedResult::Completeable(1, b""),
                b'i' => FeedResult::Completeable(9, b"n"),
                _ => FeedResult::None,
            },
            _ => FeedResult::None,
        }
    }

    fn completes(line: &[u8], rem: &[u8]) -> bool {
        line.ends_with(rem)
    }

    fn incr(line: &[u8]) -> (u8, &[u8]) {
        (line[line.len() - 1], &line[..line.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day01_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((209, 198).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((54_390, 54_277).into_day_result(), solution);
    }
}
