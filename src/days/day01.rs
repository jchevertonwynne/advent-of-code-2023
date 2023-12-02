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
    const NUMS_FORWARD: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    finder(line.iter().cloned(), NUMS_FORWARD)
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
