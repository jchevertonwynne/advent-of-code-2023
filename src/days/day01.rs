use arrayvec::ArrayVec;
use bstr::ByteSlice;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.as_bytes().lines() {
        let (p1f, p2f) = first_forward(line);
        let (p1b, p2b) = first_backward(line);
        let num1 = p1f * 10 + p1b;
        let num2 = p2f * 10 + p2b;
        p1 += num1;
        p2 += num2;
    }

    (p1, p2).into_result()
}

fn first_forward(line: &[u8]) -> (usize, usize) {
    let mut found_p1 = false;
    let mut found_p2 = false;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut a1: ArrayVec<State, 9> = ArrayVec::<State, 9>::from_iter([State(0)]);

    for b in line {
        match b {
            b'1'..=b'9' => {
                a1 = ArrayVec::from_iter([State(0)]);

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
                let mut a2 = ArrayVec::from_iter([State(0)]);

                while let Some(s) = a1.pop() {
                    match s.feed(*b) {
                        FeedResult::None => {}
                        FeedResult::One(a) => a2.push(a),
                        FeedResult::Complete(n) => {
                            p2 = n;
                            found_p2 = true;
                            break;
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

fn first_backward(line: &[u8]) -> (usize, usize) {
    const NUMS_BACKWARD: [&str; 9] = [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    finder(line.iter().rev().cloned(), NUMS_BACKWARD)
}

fn finder(line: impl Iterator<Item = u8>, searches: [&'static str; 9]) -> (usize, usize) {
    let mut found_p1 = false;
    let mut found_p2 = false;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut incrementors = searches.map(|n| Incrementor {
        src: n.as_bytes(),
        index: 0,
    });

    for b in line {
        if found_p1 && found_p2 {
            break;
        }
        match b {
            b'1'..=b'9' => {
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
                for (i, incrementor) in incrementors.iter_mut().enumerate() {
                    if incrementor.feed(b) {
                        p2 = i + 1;
                        found_p2 = true;
                        break;
                    }
                }
            }
        }
    }

    (p1, p2)
}

struct Incrementor {
    src: &'static [u8],
    index: usize,
}

impl Incrementor {
    fn feed(&mut self, b: u8) -> bool {
        if self.src[self.index] == b {
            self.index += 1;
        } else if self.src[0] == b {
            self.index = 1;
        } else {
            self.index = 0;
        }
        self.index == self.src.len()
    }
}

#[derive(Debug, Clone, Copy)]
struct State(u8);

impl State {
    fn feed(self, b: u8) -> FeedResult {
        match self.0 {
            // first layer
            0 => match b {
                b'o' => FeedResult::One(State(1)),
                b't' => FeedResult::One(State(2)),
                b'f' => FeedResult::One(State(3)),
                b's' => FeedResult::One(State(4)),
                b'e' => FeedResult::One(State(5)),
                b'n' => FeedResult::One(State(6)),
                _ => FeedResult::None,
            },
            // second layer
            1 => match b {
                b'n' => FeedResult::One(State(7)),
                _ => FeedResult::None,
            },
            2 => match b {
                b'w' => FeedResult::One(State(8)),
                b'h' => FeedResult::One(State(9)),
                _ => FeedResult::None,
            },
            3 => match b {
                b'o' => FeedResult::One(State(10)),
                b'i' => FeedResult::One(State(11)),
                _ => FeedResult::None,
            },
            4 => match b {
                b'i' => FeedResult::One(State(12)),
                b'e' => FeedResult::One(State(13)),
                _ => FeedResult::None,
            },
            5 => match b {
                b'i' => FeedResult::One(State(14)),
                _ => FeedResult::None,
            },
            6 => match b {
                b'i' => FeedResult::One(State(15)),
                _ => FeedResult::None,
            },
            // second layer
            7 => match b {
                b'e' => FeedResult::Complete(1),
                _ => FeedResult::None,
            },
            8 => match b {
                b'o' => FeedResult::Complete(2),
                _ => FeedResult::None,
            },
            9 => match b {
                b'r' => FeedResult::One(State(18)),
                _ => FeedResult::None,
            },
            10 => match b {
                b'u' => FeedResult::One(State(19)),
                _ => FeedResult::None,
            },
            11 => match b {
                b'v' => FeedResult::One(State(20)),
                _ => FeedResult::None,
            },
            12 => match b {
                b'x' => FeedResult::Complete(6),
                _ => FeedResult::None,
            },
            13 => match b {
                b'v' => FeedResult::One(State(22)),
                _ => FeedResult::None,
            },
            14 => match b {
                b'g' => FeedResult::One(State(23)),
                _ => FeedResult::None,
            },
            15 => match b {
                b'n' => FeedResult::One(State(24)),
                _ => FeedResult::None,
            },
            //third layer
            18 => match b {
                b'e' => FeedResult::One(State(25)),
                _ => FeedResult::None,
            },
            19 => match b {
                b'r' => FeedResult::Complete(4),
                _ => FeedResult::None,
            },
            20 => match b {
                b'e' => FeedResult::Complete(5),
                _ => FeedResult::None,
            },
            22 => match b {
                b'e' => FeedResult::One(State(28)),
                _ => FeedResult::None,
            },
            23 => match b {
                b'h' => FeedResult::One(State(29)),
                _ => FeedResult::None,
            },
            24 => match b {
                b'e' => FeedResult::Complete(9),
                _ => FeedResult::None,
            },
            // fourth layer
            25 => match b {
                b'e' => FeedResult::Complete(3),
                _ => FeedResult::None,
            },
            28 => match b {
                b'n' => FeedResult::Complete(7),
                _ => FeedResult::None,
            },
            29 => match b {
                b't' => FeedResult::Complete(8),
                _ => FeedResult::None,
            },
            _ => FeedResult::None,
        }
    }
}

enum FeedResult {
    None,
    One(State),
    Complete(usize),
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
